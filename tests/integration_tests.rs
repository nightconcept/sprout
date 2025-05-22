use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use tempfile::TempDir;
use tempfile::tempdir;

fn create_temp_bundle_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp bundle file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp bundle file");
    file.flush().expect("Failed to flush temp bundle file");
    file.as_file()
        .sync_all()
        .expect("Failed to sync temp bundle file to disk");
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
        predicate::str::contains("Usage: sprout.exe") // Expect clap's help message
            .and(predicate::str::contains("Arguments:"))
            .and(predicate::str::contains("Options:")),
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
fn test_output_path_flag_takes_precedence_over_positional() -> Result<(), Box<dyn std::error::Error>>
{
    let bundle_content = "================================================\nFile: flag_precedence.txt\n================================================\nFlag precedence test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let positional_output_dir = TempDir::new()?; // This should be ignored
    let flag_output_dir = TempDir::new()?; // This should be used

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path())
        .arg(positional_output_dir.path()) // Positional output
        .arg("-o")
        .arg(flag_output_dir.path()); // Flag output

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            flag_output_dir.path().display() // Expect flag path in success message
        )));

    let file_path_in_flag_dir = flag_output_dir.path().join("flag_precedence.txt");
    assert!(file_path_in_flag_dir.exists());
    assert_eq!(
        fs::read_to_string(file_path_in_flag_dir)?,
        "Flag precedence test\n"
    );

    // Ensure nothing was written to the positional_output_dir
    assert!(
        fs::read_dir(positional_output_dir.path())?.next().is_none(),
        "Positional output directory should be empty"
    );

    Ok(())
}

#[test]
fn test_main_success_path_standard_args() -> Result<(), Box<dyn std::error::Error>> {
    let bundle_content = "================================================\nFile: main_success.txt\n================================================\nMain success test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg(bundle_file.path()).arg(output_dir.path()); // Standard positional args

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.",
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("main_success.txt");
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path)?, "Main success test\n");

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

#[test]
fn test_input_flag_output_positional_valid_bundle() -> Result<(), Box<dyn std::error::Error>> {
    // RENAMED from test_bundle_file_no_valid_entries
    // This test checks for successful sprouting with
    // input flag (-i) and positional output directory.
    // This is intended to cover specific branches in main.rs for argument handling (lines 44, 51, 76).

    let bundle_content = "================================================\nFile: new_file_for_coverage.txt\n================================================\nContent for coverage test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg("-i") // Use input flag
        .arg(bundle_file.path())
        .arg(output_dir.path()); // Use positional output directory

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Successfully sprouted 1 file(s) from '{}' to '{}'.", // Check for success message (main.rs line 76)
            bundle_file.path().display(),
            output_dir.path().display()
        )));

    let file_path = output_dir.path().join("new_file_for_coverage.txt");
    assert!(file_path.exists(), "Expected file was not created");
    assert_eq!(
        fs::read_to_string(file_path)?,
        "Content for coverage test\n"
    );

    Ok(())
}

#[test]
fn test_input_flag_and_force_flag_for_collision_bypass() -> Result<(), Box<dyn std::error::Error>> {
    // Specifically targets line 51 in main.rs where force flag is checked for bypassing collisions
    let bundle_content = "================================================\nFile: input_and_force.txt\n================================================\nInput flag and force flag test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    // Create a file that will be overwritten
    let existing_file_path = output_dir.path().join("input_and_force.txt");
    fs::write(&existing_file_path, "Original content")?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg("-i")
        .arg(bundle_file.path())
        .arg("-o")
        .arg(output_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("(files overwritten if necessary)"));

    // Verify the file was overwritten
    assert!(existing_file_path.exists());
    assert_eq!(
        fs::read_to_string(existing_file_path)?,
        "Input flag and force flag test\n"
    );

    Ok(())
}

#[test]
fn test_input_flag_with_force_flag_message() -> Result<(), Box<dyn std::error::Error>> {
    // This test ensures the force flag message appears in the output (line 76 in main.rs)
    // Also tests using input flag instead of positional arg (related to line 51)
    let bundle_content = "================================================\nFile: force_flag_message.txt\n================================================\nForce flag message test\n";
    let bundle_file = create_temp_bundle_file(bundle_content);
    let output_dir = TempDir::new()?;

    // Create a file that will be overwritten
    let existing_file_path = output_dir.path().join("force_flag_message.txt");
    fs::write(&existing_file_path, "Original content")?;

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.arg("-i")
        .arg(bundle_file.path())
        .arg("-o")
        .arg(output_dir.path())
        .arg("--force");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("(files overwritten if necessary)"));

    // Verify the file was overwritten
    assert!(existing_file_path.exists());
    assert_eq!(
        fs::read_to_string(existing_file_path)?,
        "Force flag message test\n"
    );

    Ok(())
}

#[test]
fn test_single_dot_argument_as_bundle_path() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    // We don't create a file named ".". We use the directory itself.

    let mut cmd = Command::cargo_bin("sprout")?;
    cmd.current_dir(temp_dir.path()); // Run `sprout` from within the temp_dir
    cmd.arg("."); // Use "." (the current directory) as the bundle path

    cmd.assert()
        .failure() // Expect the command to fail
        .stderr(
            predicate::str::contains("Failed to read bundle file").and(
                predicate::str::contains("Is a directory") // Unix-like
                    .or(predicate::str::contains("Access is denied.")), // Windows
            ),
        ); // Check for the specific error

    Ok(())
}
