// src/bundler.rs
// Module for file/directory creation and output logic

use crate::parser::ParsedEntry;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Creates directories and files based on the parsed bundle entries.
///
/// This function is called only if bundle parsing and collision checks pass.
/// For each `ParsedEntry`:
///   - Resolves the full absolute path for the new file.
///   - Ensures its parent directory exists using `std::fs::create_dir_all(parent_path)`.
///   - Writes the `entry.content` to the file path using `std::fs::write`.
///
/// Handles potential I/O errors during directory/file creation gracefully, returning an `anyhow::Error`.
pub fn create_files_from_bundle(entries: &[ParsedEntry], output_dir: &Path) -> Result<()> {
    for entry in entries {
        let full_target_path = output_dir.join(&entry.path);

        if let Some(parent_path) = full_target_path.parent() {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path).with_context(|| {
                    format!("Failed to create parent directory: {:?}", parent_path)
                })?;
            }
        }

        fs::write(&full_target_path, &entry.content)
            .with_context(|| format!("Failed to write file: {:?}", full_target_path))?;
    }
    Ok(())
}

/// Checks for path collisions in the output directory.
///
/// For each `ParsedEntry`, it constructs the full target path by joining
/// `output_dir` and `entry.path`. It then checks if this full target path
/// already exists. If any collisions are detected, it returns an `anyhow::Error`
/// detailing all collisions.
pub fn check_for_collisions(entries: &[ParsedEntry], output_dir: &Path) -> Result<()> {
    let mut collisions = Vec::new();

    for entry in entries {
        let target_path = output_dir.join(&entry.path);
        if target_path.exists() {
            collisions.push(target_path);
        } else {
            let mut current_check_path = PathBuf::new();
            for component in entry
                .path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .components()
            {
                current_check_path.push(component);
                let full_component_path = output_dir.join(&current_check_path);
                if full_component_path.is_file()
                    && entry
                        .path
                        .strip_prefix(&current_check_path)
                        .is_ok_and(|p| !p.as_os_str().is_empty())
                {
                    collisions.push(full_component_path);
                    break;
                }
            }
        }
    }

    if !collisions.is_empty() {
        let collision_details = collisions
            .iter()
            .map(|p| format!("  - {}", p.display()))
            .collect::<Vec<String>>()
            .join("\n");
        return Err(anyhow::anyhow!(
            "Output path collision detected. The following paths already exist or conflict with directory creation:\n{}",
            collision_details
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ParsedEntry;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn create_parsed_entry(path_str: &str, content_str: &str) -> ParsedEntry {
        ParsedEntry {
            path: PathBuf::from(path_str),
            content: String::from(content_str),
        }
    }

    #[test]
    fn test_check_for_collisions_no_collision() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        let entries = vec![
            create_parsed_entry("file1.txt", "content1"),
            create_parsed_entry("dir1/file2.txt", "content2"),
        ];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_for_collisions_single_file_collision() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        File::create(output_dir.join("file1.txt")).unwrap();

        let entries = vec![
            create_parsed_entry("file1.txt", "content1"),
            create_parsed_entry("file2.txt", "content2"),
        ];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_err());
        let error_message = result.err().unwrap().to_string();
        assert!(error_message.contains("Output path collision detected"));
        assert!(error_message.contains(&output_dir.join("file1.txt").display().to_string()));
    }

    #[test]
    fn test_check_for_collisions_multiple_file_collisions() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        File::create(output_dir.join("file1.txt")).unwrap();
        fs::create_dir_all(output_dir.join("dir1")).unwrap();
        File::create(output_dir.join("dir1/file2.txt")).unwrap();

        let entries = vec![
            create_parsed_entry("file1.txt", "c1"),
            create_parsed_entry("dir1/file2.txt", "c2"),
            create_parsed_entry("file3.txt", "c3"),
        ];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_err());
        let error_message = result.err().unwrap().to_string();
        assert!(error_message.contains(&output_dir.join("file1.txt").display().to_string()));
        assert!(error_message.contains(&output_dir.join("dir1/file2.txt").display().to_string()));
    }

    #[test]
    fn test_check_for_collisions_directory_as_file_collision() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        fs::create_dir_all(output_dir.join("item")).unwrap();

        let entries = vec![create_parsed_entry("item", "content")];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_err());
        let error_message = result.err().unwrap().to_string();
        assert!(error_message.contains(&output_dir.join("item").display().to_string()));
    }

    #[test]
    fn test_check_for_collisions_file_as_directory_collision() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        File::create(output_dir.join("item")).unwrap();

        let entries = vec![create_parsed_entry("item/another.txt", "content")];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_err());
        let error_message = result.err().unwrap().to_string();
        assert!(error_message.contains(&output_dir.join("item").display().to_string()));
        assert!(error_message.contains("conflict with directory creation"));
    }

    #[test]
    fn test_check_for_collisions_deep_file_as_directory_collision() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path();
        fs::create_dir_all(output_dir.join("level1")).unwrap();
        File::create(output_dir.join("level1/item")).unwrap();

        let entries = vec![create_parsed_entry("level1/item/another.txt", "content")];

        let result = check_for_collisions(&entries, output_dir);
        assert!(result.is_err());
        let error_message = result.err().unwrap().to_string();
        assert!(error_message.contains(&output_dir.join("level1/item").display().to_string()));
    }

    #[test]
    fn test_create_single_file() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path();
        let entries = vec![create_parsed_entry("file1.txt", "Hello World")];

        create_files_from_bundle(&entries, output_dir)?;

        let file_path = output_dir.join("file1.txt");
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(file_path)?, "Hello World");
        Ok(())
    }

    #[test]
    fn test_create_multiple_files() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path();
        let entries = vec![
            create_parsed_entry("file1.txt", "Content 1"),
            create_parsed_entry("file2.txt", "Content 2"),
        ];

        create_files_from_bundle(&entries, output_dir)?;

        let file_path1 = output_dir.join("file1.txt");
        assert!(file_path1.exists());
        assert_eq!(fs::read_to_string(file_path1)?, "Content 1");

        let file_path2 = output_dir.join("file2.txt");
        assert!(file_path2.exists());
        assert_eq!(fs::read_to_string(file_path2)?, "Content 2");
        Ok(())
    }

    #[test]
    fn test_create_files_in_nested_directories() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path();
        let entries = vec![
            create_parsed_entry("dir1/file1.txt", "Nested Content 1"),
            create_parsed_entry("dir1/dir2/file2.txt", "Deeply Nested Content 2"),
            create_parsed_entry("file3.txt", "Root Content 3"),
        ];

        create_files_from_bundle(&entries, output_dir)?;

        let path1 = output_dir.join("dir1/file1.txt");
        assert!(path1.exists());
        assert_eq!(fs::read_to_string(path1)?, "Nested Content 1");
        assert!(output_dir.join("dir1").is_dir());

        let path2 = output_dir.join("dir1/dir2/file2.txt");
        assert!(path2.exists());
        assert_eq!(fs::read_to_string(path2)?, "Deeply Nested Content 2");
        assert!(output_dir.join("dir1/dir2").is_dir());

        let path3 = output_dir.join("file3.txt");
        assert!(path3.exists());
        assert_eq!(fs::read_to_string(path3)?, "Root Content 3");
        Ok(())
    }

    #[test]
    fn test_create_file_with_empty_content() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path();
        let entries = vec![create_parsed_entry("empty.txt", "")];

        create_files_from_bundle(&entries, output_dir)?;

        let file_path = output_dir.join("empty.txt");
        assert!(file_path.exists());
        assert_eq!(fs::read_to_string(file_path)?, "");
        Ok(())
    }

    #[test]
    fn test_create_files_complex_paths_and_content() -> Result<()> {
        let dir = tempdir()?;
        let output_dir = dir.path();
        let entries = vec![
            create_parsed_entry("src/main.rs", "fn main() {\n    println!(\"Hello\");\n}"),
            create_parsed_entry("docs/README.md", "# My Project\n\nThis is a test."),
            create_parsed_entry("config/settings.toml", "key = \"value\"\nnumber = 123"),
        ];

        create_files_from_bundle(&entries, output_dir)?;

        let path_rs = output_dir.join("src/main.rs");
        assert!(path_rs.exists());
        assert_eq!(
            fs::read_to_string(path_rs)?,
            "fn main() {\n    println!(\"Hello\");\n}"
        );
        assert!(output_dir.join("src").is_dir());

        let path_md = output_dir.join("docs/README.md");
        assert!(path_md.exists());
        assert_eq!(
            fs::read_to_string(path_md)?,
            "# My Project\n\nThis is a test."
        );
        assert!(output_dir.join("docs").is_dir());

        let path_toml = output_dir.join("config/settings.toml");
        assert!(path_toml.exists());
        assert_eq!(
            fs::read_to_string(path_toml)?,
            "key = \"value\"\nnumber = 123"
        );
        assert!(output_dir.join("config").is_dir());

        Ok(())
    }
}
