use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use tempfile::TempDir;

fn create_temp_bundle_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp bundle file");
    write!(file, "{}", content).expect("Failed to write to temp bundle file");
    file
}

#[test]
fn test_valid_bundle_simple_structure() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: file1.txt\n================================================\nHello from file1\n================================================\nFile: file2.txt\n================================================\nContent of file2\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 2 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file1_path = output_dir.path().join("file1.txt");
    let file2_path = output_dir.path().join("file2.txt");

    assert!(file1_path.exists());
    assert_eq!(fs::read_to_string(file1_path)?, "Hello from file1\n");

    assert!(file2_path.exists());
    assert_eq!(fs::read_to_string(file2_path)?, "Content of file2\n");

    Ok(())
}

#[test]
fn test_valid_bundle_nested_structure() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: dir1/file1.txt\n================================================\nNested content\n================================================\nFile: dir1/dir2/file2.txt\n================================================\nDeeply nested\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 2 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file1_path = output_dir.path().join("dir1/file1.txt");
    let file2_path = output_dir.path().join("dir1/dir2/file2.txt");

    assert!(file1_path.exists());
    assert_eq!(fs::read_to_string(file1_path)?, "Nested content\n");

    assert!(file2_path.exists());
    assert_eq!(fs::read_to_string(file2_path)?, "Deeply nested\n");

    Ok(())
}

#[test]
fn test_malformed_bundle_file() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: file1.txt\nThis is not a valid header\nContent\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert().failure().stderr(
        predicate::str::contains("Bundle parsing failed")
            .and(predicate::str::contains("Malformed file header")),
    );

    assert!(fs::read_dir(output_dir.path())?.next().is_none());

    Ok(())
}

#[test]
fn test_output_file_collision() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: existing_file.txt\n================================================\nSome content\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let conflicting_file_path = output_dir.path().join("existing_file.txt");
    fs::write(&conflicting_file_path, "Original content")?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert().failure().stderr(
        predicate::str::contains("Output path collision detected").and(predicate::str::contains(
            conflicting_file_path.to_str().unwrap(),
        )),
    );

    assert_eq!(
        fs::read_to_string(conflicting_file_path)?,
        "Original content"
    );

    Ok(())
}

#[test]
fn test_empty_bundle_file() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_file = create_temp_bundle_file("");
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Bundle file '{}' is empty or contains no valid entries. Nothing to sprout.",
            bundle_file.path().display()
        )));

    assert!(fs::read_dir(output_dir.path())?.next().is_none());
    Ok(())
}

#[test]
fn test_output_to_current_directory_default() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: default_dir_file.txt\n================================================\nDefault dir test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);

    let current_dir_scope = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.current_dir(current_dir_scope.path())
        .arg(bundle_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            "."
        )));

    let file_path = current_dir_scope.path().join("default_dir_file.txt");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path)?, "Default dir test\n");

    Ok(())
}

#[test]
fn test_output_to_specified_directory_via_positional_arg() -> Result<(), Box<dyn std::error::Error>>
{
    let bundle_content = "================================================\nFile: specified_pos_file.txt\n================================================\nSpecified dir test - positional\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("specified_pos_file.txt");
    assert!(file_path.exists());
    assert_eq!(
        fs::read_to_string(file_path)?,
        "Specified dir test - positional\n"
    );

    Ok(())
}

#[test]
fn test_output_to_specified_directory_via_o_flag() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: specified_flag_file.txt\n================================================\nSpecified dir test - flag\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg("-o").arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("specified_flag_file.txt");
    assert!(file_path.exists());
    assert_eq!(
        fs::read_to_string(file_path)?,
        "Specified dir test - flag\n"
    );

    Ok(())
}

#[test]
fn test_input_via_i_flag() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: input_flag_test.txt\n================================================\nInput via -i flag\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg("-i")
        .arg(bundle_file.path())
        .arg("-o")
        .arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("input_flag_test.txt");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path)?, "Input via -i flag\n");

    Ok(())
}

#[test]
fn test_missing_input_bundle() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.assert().failure().stderr(
        predicate::str::contains("error: the following required arguments were not provided:")
            .and(predicate::str::contains("<BUNDLE_FILE_PATH>")),
    );
    Ok(())
}

#[test]
fn test_bundle_with_empty_file_content() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: empty_file.txt\n================================================\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("empty_file.txt");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path)?, "");

    Ok(())
}

#[test]
fn test_force_overwrite_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: overwrite_me.txt\n================================================\nNew Content\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;
    let target_file_path = output_dir.path().join("overwrite_me.txt");

    // Create the file initially
    fs::write(&target_file_path, "Old Content")?;
    assert_eq!(fs::read_to_string(&target_file_path)?, "Old Content");

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path())
        .arg(output_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'. (files overwritten if necessary)",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    assert!(target_file_path.exists());
    assert_eq!(fs::read_to_string(target_file_path)?, "New Content\n");

    Ok(())
}

#[test]
fn test_force_overwrite_existing_file_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: overwrite_me_short.txt\n================================================\nNew Content Short\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;
    let target_file_path = output_dir.path().join("overwrite_me_short.txt");

    // Create the file initially
    fs::write(&target_file_path, "Old Content Short")?;
    assert_eq!(fs::read_to_string(&target_file_path)?, "Old Content Short");

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path()).arg("-f"); // Short flag for force

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'. (files overwritten if necessary)",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    assert!(target_file_path.exists());
    assert_eq!(fs::read_to_string(target_file_path)?, "New Content Short\n");

    Ok(())
}

#[test]
fn test_force_still_fails_if_parent_is_file() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: existing_file_as_parent/new_child.txt\n================================================\nShould not be created\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    // Create a file that would be a parent directory
    let conflicting_parent_path = output_dir.path().join("existing_file_as_parent");
    fs::write(&conflicting_parent_path, "I am a file.")?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path())
        .arg(output_dir.path())
        .arg("--force");

    cmd.assert().failure().stderr(
        predicate::str::contains("Failed to create parent directory")
            .or(
                // Error from create_dir_all
                predicate::str::contains("its parent")
                    .and(predicate::str::contains("is an existing file")), // Error from bundler.rs explicit check
            )
            .or(
                predicate::str::contains("Failed to write file"), // Error from fs::write if parent is a file
            ),
    );

    // Ensure original file is untouched and no new file/directory was created under/as it
    assert_eq!(
        fs::read_to_string(&conflicting_parent_path)?,
        "I am a file."
    );
    assert!(
        !output_dir
            .path()
            .join("existing_file_as_parent/new_child.txt")
            .exists()
    );
    assert!(conflicting_parent_path.is_file());

    Ok(())
}
