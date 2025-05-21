# codesprout - Task List

## Milestone 0: Development Environment & Workflow Setup

**Goal:** Establish a consistent Rust development environment using `mise` and define commands/scripts for essential development tasks such as formatting, linting, building, and testing the `sprout` CLI.

- [x] **Task 0.1: Setup `mise` for Rust Version Management**
  - [x] Create a `mise.toml` file in the project root.
  - [x] Specify the Rust version to be used for the project (e.g., `rust = "latest"` or a specific version like `rust = "1.87"` to align with PRD).
  - [x] Verification: After navigating to the project directory in a new terminal, `mise current rust` (or `mise which rustc`) shows the correct Rust compiler path and version as specified in `.mise.toml`. Running `rustc --version` confirms the active version.

- [x] **Task 0.2: Define Code Formatting Task & Integration**
  - [x] Document the command for checking formatting: `cargo fmt --all --check`.
  - [x] Document the command for applying formatting: `cargo fmt --all`.
  - [x] (Optional) Consider integrating `cargo fmt --all --check` into a pre-commit hook or CI step later.
    - [x] Verification: `cargo fmt --all --check` passes on a cleanly formatted codebase. `cargo fmt --all` correctly formats any misformatted Rust files.
- [x] **Task 0.3: Define Code Linting Task & Integration**
  - [x] Document the command for linting: `cargo clippy --all-targets -- -D warnings` (treat all warnings as errors).
  - [x] (Optional) Consider integrating this lint check into a pre-commit hook or CI step later.
  - [x] Verification: `cargo clippy --all-targets -- -D warnings` passes on a lint-free codebase.
- [ ] **Task 0.4: Define Building Tasks**
  - [x] Document command for debug builds: `cargo build`.
  - [ ] Document command for optimized release builds: `cargo build --release`.
  - [x] Note: `Cargo.toml` should be configured with release profile optimizations as per PRD (e.g., `opt-level = "z"`, `lto = true`, `panic = "abort"`, `strip = true`).
  - [x] Verification: `cargo build` successfully compiles the project. `cargo build --release` successfully compiles the project and produces an optimized binary.
- [x] **Task 0.5: Define Testing Task**  
    - [x] Document command for running all tests: `cargo test`.
    - [x] Document command for running tests with more output: `cargo test -- --nocapture` (if needed for debugging).
    - [x] Verification: `cargo test` runs all available tests and reports pass/fail status (initially, this might be an empty test suite or auto-generated tests).

## Milestone 1: Project Initialization & CLI Argument Parsing

**Goal:** Initialize the Rust project structure for `sprout` (if not fully covered by Milestone 0 tasks related to `cargo new`), implement robust command-line argument parsing using `clap`, and set up the basic application structure to handle the main processing flow.

- [x] **Task 1.1: Complete Rust Project Initialization**
    
    - [x] Run `cargo new sprout --bin` (if not done as part of `mise` setup or if a fresh start is preferred).
    - [x] Ensure `Cargo.toml` is correctly configured:
        - Set `name = "sprout"` (as the binary will be `sprout`), `description = "A CLI tool to sprout files from a bundle."`, `authors = ["Your Name <you@example.com>"]`, `edition = "2024"`, `version = "0.1.0"`.
        - Add `clap` as a dependency with the "derive" feature (e.g., `clap = { version = "4.x", features = ["derive"] }`).
        - Add `anyhow` as a dependency (e.g., `anyhow = "1.x"`).
    - [x] Verification: Project compiles successfully (`cargo build`). `Cargo.toml` reflects the specified settings and dependencies. `sprout --version` (once version is integrated with `clap`'s `App::version`) works.
- [x] **Task 1.2: Implement CLI Argument Parsing with `clap`**
    
    - [x] In `src/main.rs`, define a struct (e.g., `CliArgs`) using `clap::Parser` to manage command-line arguments.
    - [x] Implement parsing for:
        - `bundle_file_path` (positional, `PathBuf`, required unless `-i` is used).
        - `output_directory_path` (positional, `PathBuf`, optional, defaults to current directory unless `-o` is used).
        - `-i, --input <PATH>`: Optional flag for bundle file path (`Option<PathBuf>`).
        - `-o, --output <PATH>`: Optional flag for output directory path (`Option<PathBuf>`).
    - [x] Logic to determine effective input and output paths (handling defaults and overrides).
    - [x] Ensure input path is effectively mandatory.
    - [x] In `main()`, parse the arguments using `CliArgs::parse()`.
    - [x] Verification:
        - `sprout --help` displays correctly formatted help message with all arguments and options.
        - `sprout valid_bundle.txt` correctly identifies `valid_bundle.txt` as input and uses `.` as default output.
        - `sprout -i valid_bundle.txt -o ./my_output` correctly parses input and output paths.
        - `sprout ./my_output_dir` (assuming `my_output_dir` is not the bundle file) is handled by clap for positional args.
        - `sprout` (with no input arguments) shows an error message from `clap` indicating missing required input.
- [x] **Task 1.3: Establish Main Application Logic Flow & Error Handling**
    
    - [x] In `src/main.rs`, ensure `main` function returns `anyhow::Result<()>`.
    - [x] Define the high-level steps based on parsed arguments:
        1. Resolve final input and output paths.
        2. (Stub) Call `parser::process_bundle_file(input_path)`.
        3. (Stub) If parsing/validation successful, call `bundler::create_files(parsed_data, output_path)`.
    - [x] Implement basic stubs for these functions in their respective (future) modules (`parser.rs`, `bundler.rs`).
    - [x] Ensure `main` propagates errors from these calls using `?`.
    - [x] Verification: The `sprout` command runs, prints placeholder messages for each stubbed step based on parsed arguments, and exits gracefully (or with a placeholder error from a stub).

## Milestone 2: Bundle File Parsing and Validation

**Goal:** Implement the complete logic for reading, parsing, and validating the `digest.txt`-style bundle file. The tool should be able to identify all format errors in the bundle file before any file system operations are attempted.

- [x] **Task 2.1: Develop Bundle File Parser (`src/parser.rs`)**
    
    - [x] Create the `src/parser.rs` module. (Covered by previous tasks or implicitly by creating the file)
    - [x] Define a public function, e.g., `parse_bundle(bundle_path: &Path) -> anyhow::Result<Vec<ParsedEntry>>`.
    - [x] Implement logic to read the content of the bundle file.
    - [x] Implement parsing logic to iterate through the file content, recognizing the `================================================\nFile: path/to/file.ext\n================================================\n...content...` structure.
    - [x] Extract the relative file path (`String` or `PathBuf`) and the multi-line content (`String`) for each entry.
    - [x] Store the parsed data in a struct, e.g., `pub struct ParsedEntry { pub path: PathBuf, pub content: String }`.
    - [x] Verification: Unit tests for the `parser::parse_bundle` function covering:
        - [x] Empty bundle file (should return empty Vec or appropriate error).
        - [x] Bundle file with one entry.
        - [x] Bundle file with multiple entries.
        - [x] Entries with empty content.
        - [x] Entries with complex multi-line content.
        - [x] Correct path extraction (including paths with subdirectories).
- [x] **Task 2.2: Implement Bundle File Format Validation**
    
    - [x] Within `src/parser.rs`, enhance the parsing or add a distinct validation step for the parsed entries and the overall bundle structure. This validation should occur before returning successfully from `parse_bundle`.
    - [x] Validation checks should include:
        - Each `File:` header line must be properly formed and contain a non-empty, valid relative path.
        - No duplicate paths within the bundle.
        - Consider edge cases: premature EOF, missing headers, content before the first header (should it be ignored or an error?). (Implemented: content before first header is an error; other structural errors handled)
    - [x] The validation should collect _all_ format errors found in the bundle and return them as a single `anyhow::Error` (possibly by formatting a list of specific error details).
    - [x] In `src/main.rs`, call `parse_bundle`. If it returns `Err`, print the error (which should now include all validation issues) and exit. (Handled by `anyhow` and `?` operator).
    - [x] Verification:
        - Update unit tests for `parser::parse_bundle` to cover various invalid bundle file scenarios (e.g., malformed `File:` line, duplicate paths, EOF within a file block).
        - Test the `sprout` CLI with sample malformed bundle files; ensure all relevant errors are reported clearly and the program exits without attempting to write files.

## Milestone 3: File System Operations, Collision Detection, and Final Integration

**Goal:** Implement the logic to create the directory structure and files as specified in the parsed bundle. This includes robust collision detection in the output directory. This milestone will result in a fully functional `sprout` CLI for its core purpose.

- [x] **Task 3.1: Implement Output Path Collision Detection (`src/bundler.rs`)**
    
    - [x] Create the `src/bundler.rs` module.
    - [x] Implement a function, e.g., `check_for_collisions(entries: &[ParsedEntry], output_dir: &Path) -> anyhow::Result<()>`.
    - [x] For each `ParsedEntry` in the list:
        - Construct the full target path by joining `output_dir` and `entry.path`.
        - Check if this full target path already exists using `std::path::Path::exists()`.
    - [x] If any path collision is detected, this function should return an `anyhow::Error` detailing all collisions found.
    - [x] In `src/main.rs`, call this collision check function after successful bundle parsing. If it returns `Err`, print the error and exit.
    - [x] Verification:
        - Unit tests for `bundler::check_for_collisions` with scenarios: no collisions, one collision, multiple collisions, collision with a file where a directory is needed, collision with a directory where a file is needed.
        - CLI Test: `sprout` aborts with an informative error if a target file path already exists.
        - CLI Test: `sprout` aborts if a parent directory to be created (e.g., `new_dir/`) conflicts with an existing file named `new_dir`.
- [x] **Task 3.2: Implement Directory and File Creation (`src/bundler.rs`)**
    
    - [x] Implement a function, e.g., `create_files_from_bundle(entries: &[ParsedEntry], output_dir: &Path) -> anyhow::Result<()>`.
    - [x] This function is called only if bundle parsing and collision checks pass.
    - [x] For each `ParsedEntry`:
        - [x] Resolve the full absolute path for the new file.
        - [x] Ensure its parent directory exists using `std::fs::create_dir_all(parent_path)`.
        - [x] Write the `entry.content` to the file path using `std::fs::write`.
    - [x] Handle potential I/O errors during directory/file creation gracefully, returning an `anyhow::Error`.
    - [x] Verification:
        - [x] Unit tests for `bundler::create_files_from_bundle` to verify:
            - [x] Creation of a single file in the output directory.
            - [x] Creation of multiple files.
            - [x] Creation of files within newly created nested subdirectories.
            - [x] Correct writing of file content.
        - (Covered by integration tests in next task mostly)
- [x] **Task 3.3: Final Integration, User Feedback, and Testing**
    
    - [x] Integrate all components in `src/main.rs`: CLI parsing (`clap`), bundle file reading/validation (`parser.rs`), collision detection, and file/directory creation (`bundler.rs`).
    - [x] Implement clear success messages (e.g., "Successfully sprouted N files to <output_directory>.").
    - [x] Ensure all error paths (bundle format errors, I/O errors, collision errors) provide user-friendly messages propagated by `anyhow`.
    - [x] Write integration tests for the `sprout` CLI (e.g., using a test runner or simple shell scripts that invoke the compiled binary):
        - [x] Test with a valid bundle file creating a simple structure.
        - [x] Test with a valid bundle file creating a nested structure.
        - [x] Test failure with a malformed bundle file (ensure all errors are printed).
        - [x] Test failure due to output file collision (ensure specific collision is reported).
        - [x] Test with empty bundle file.
        - [x] Test output to current directory (default) and to a specified directory.
    - [x] Verification: The `sprout` command works end-to-end for valid scenarios and fails gracefully with correct, comprehensive error messages for all defined error conditions. Code coverage for core logic (parsing, bundling) is reasonable.

## Additional Tasks / Backlog

(Items from the PRD's "Future Considerations" that are out of scope for this initial prototype but good to keep in mind for future development)

- [ ] Implement Reverse Operation ("Bundling" a directory into a `digest.txt` style file).
- [ ] Add file overwrite protection options (`--force`, skip, prompt).
- [ ] Introduce a configuration file for `sprout` (e.g., custom delimiters, default output dir).
- [ ] Add more comprehensive test cases for file system edge cases (permissions, symlinks, etc.).
- [ ] Refine and add more detailed verbose logging options (e.g., using `log` and `env_logger` crates).
- [ ] Research and implement packaging/distribution methods for the Rust binary (e.g., `cargo-dist`, GitHub Releases assets, AUR, Homebrew).
- [ ] Performance benchmarking and optimization for very large bundle files or a high number of files.
- [x] **Task 4.1: Create Project README** (2025-05-20) - Create a cool README.md with emojis, project description, usage, build, and test instructions.