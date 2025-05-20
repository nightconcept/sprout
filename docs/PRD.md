# codesprout - Product Requirements Document (Prototype)

## 1. Introduction

- **Project Idea:** `codesprout` is a project to develop a command-line interface (CLI) utility named `sprout`. The `sprout` tool's primary function is to parse a single, consolidated text file (referred to as a "bundle file," formatted like the provided `digest.txt` example) and "sprout" its contents into an organized directory structure with corresponding files.
- **Problem/Need:** Developers, educators, and technical writers often need to share or archive sets of code files or project snippets. Traditional methods like zip archives are not always convenient, especially for embedding into documents, wikis, or systems that primarily accept plain text. A single-file representation simplifies copy-pasting, sharing in restrictive environments, and creating self-contained examples.
- **Prototype Goal:** To build a functional `sprout` CLI in Rust that can reliably parse a `digest.txt`-style bundle file, validate its format, and recreate the described directory structure and files in a specified output location. The prototype must include clear error reporting for format issues and abort on any potential file collisions in the output directory.

## 2. Core Features / User Stories

- **Feature 1: Sprout from Bundle**
    - Description: The `sprout` CLI will read a specified bundle file (formatted like `digest.txt`), parse its content to identify individual files and their target paths, and then create these files and any necessary parent directories in the designated output location.
    - User Action(s): The user executes the `sprout` command, providing the path to the bundle file and an optional output directory.
    - Outcome(s):
        - If the bundle file is valid and no file collisions are detected in the output path, the specified directory structure and files are created as per the bundle file's content. A success message is displayed.
        - If the bundle file contains format errors, all errors are reported to the user, and no file system modifications are made.
        - If creating any file would overwrite an existing file or directory in the output path, the operation is aborted before any files are written, and an error message is displayed.
    - Command: `sprout`
    - Key Inputs:
        - Bundle File Path (required): Provided as a positional argument or via `-i <path>` / `--input <path>`.
        - Output Directory Path (optional): Provided as a positional argument or via `-o <path>` / `--output <path>`. Defaults to the current working directory.
    - Expected Output:
        - Console messages indicating files being processed (optional, perhaps under a verbose flag in the future, but basic status for prototype).
        - Clear success message upon completion.
        - Detailed error messages if bundle file validation fails or if file collisions are detected.
        - The recreated files and directories in the specified output location.

## 3. Technical Specifications

- **Primary Language(s):** Rust (latest stable version available at the time of development, e.g., 1.7X.X).
- **Key Frameworks/Libraries:**
    - `clap`: For parsing CLI arguments.
    - `anyhow`: (Recommended) For application-level error handling and reporting.
- **Database (if any):** None for this prototype.
- **Key APIs/Integrations (if any):** None.
- **Deployment Target (if applicable for prototype):** Local native executable for common desktop platforms (Linux, macOS, Windows).
- **High-Level Architectural Approach:**
    - A CLI application built in Rust.
    - Core logic will be separated into modules:
        - One module for parsing the `digest.txt`-style bundle file.
        - One module for handling file system operations (directory creation, file writing, collision detection).
        - Error handling will be centralized, potentially using custom error types or `anyhow` for context.
    - The process flow will be: 1. Parse arguments. 2. Read and fully validate the input bundle file. 3. Check for output collisions. 4. If all checks pass, create directories and files.
- **Critical Technical Decisions/Constraints:**
    - The input bundle file format is strictly the `digest.txt` style (multi-file concatenation with `================================================\nFile: path/to/file.ext\n================================================\n...content...` delimiters).
    - The `sprout` tool will perform a full analysis of the input bundle file for any format errors before attempting any file system modifications. If errors are found, they will be reported, and the tool will exit.
    - The tool will check for potential file/directory collisions in the target output directory before writing any files. If any collision is detected, the operation will be aborted with an error message, and no files will be written.

## 4. Project Structure (Optional)

A standard Rust binary (application) project structure will be used, generated initially by `cargo new sprout --bin`.

```
/codesprout_project_root
  ├── .git/
  ├── .github/              # For GitHub Actions, issue templates, etc.
  ├── docs/
  │   ├── PRD.md            # This document
  │   └── TASKS.md          # Task list
  ├── src/
  │   ├── main.rs           # CLI entry point, argument parsing (clap), main logic flow
  │   ├── parser.rs         # Module for parsing the bundle file
  │   ├── bundler.rs        # Module for file/directory creation and output logic (renamed from sprouter for clarity)
  │   └── error.rs          # (Optional) Module for custom error types if not solely relying on anyhow
  ├── Cargo.toml            # Rust project manifest, dependencies
  ├── Cargo.lock            # Generated lockfile
  ├── README.md             # Project README
  └── target/               # Build artifacts (ignored by git)
```

- `src/`: Contains all Rust source code.
    - `main.rs`: Handles CLI argument parsing using `clap` and orchestrates the overall process.
    - `parser.rs`: Responsible for reading and validating the `digest.txt`-style bundle file format.
    - `bundler.rs`: Handles the creation of directories and files based on the parsed bundle, including collision checks.
- `docs/`: Contains project documentation.
- `Cargo.toml`: Defines project metadata, dependencies (like `clap`, `anyhow`), and profiles (e.g., for release optimization).

## 5. File Descriptions (If applicable)

- **Input Bundle File (e.g., `my_bundle.txt`, `project.digest`)**:
    - Purpose: A single text file containing the content of multiple source files or text-based project structures, along with their intended relative paths.
    - Format: Plain text. Each embedded file is demarcated by a header `================================================\nFile: path/to/file.ext\n================================================\n` followed by its content. The `path/to/file.ext` specifies the relative path where the file should be created in the output directory.
    - Key Contents: A sequence of file path specifications and their corresponding multi-line text content.

## 6. Future Considerations / Out of Scope (for this prototype)

- **Out of Scope for Prototype:**
    - **Reverse Operation ("Bundling"):** Creating a bundle file from an existing directory structure.
    - **Advanced Overwrite Options:** No `--force` flag or interactive prompts to overwrite files. The prototype will only abort on collision.
    - **Configuration File:** No external configuration for `sprout` (e.g., to customize delimiters or behavior).
    - **Ignore Patterns:** No functionality to ignore specific files or patterns during a (future) bundling operation.
    - **Complex Format Validation Beyond Basic Structure:** While basic structural validation (presence of delimiters, parsable paths) is in scope, deep semantic analysis of the content within files is not.
    - **Watching files or live updates.**
- **Potential Future Enhancements (Post-Prototype):**
    - Implement the reverse "bundling" operation.
    - Add file overwrite protection options (`--force`, skip, prompt).
    - Introduce a configuration file for `sprout`.
    - Support for ignore patterns (like `.gitignore`) during bundling.
    - Stricter validation options for bundle files.
    - Support for different bundle formats or custom delimiters via configuration.

## 7. Project-Specific Coding Rules (Optional)

- **Language Version:** Rust (latest stable version at the time of development).
- **Formatting:** `rustfmt` is mandatory. Code should be formatted using `cargo fmt` before committing.
- **Linting:** `clippy` is mandatory. Code should pass `cargo clippy --all-targets -- -D warnings` (fail on warnings).
- **Error Handling:**
    - Use Rust's standard `Result<T, E>` for all functions that can produce an error.
    - Utilize the `?` operator for concise error propagation.
    - Employ the `anyhow` crate for creating and managing application-level errors, providing context, and simplifying error returns from `main()`.
- **Dependencies (`Cargo.toml`):**
    - `clap` will be used for CLI argument parsing.
    - `anyhow` will be used for error handling.
    - Other external crate dependencies should be minimized and require justification for inclusion in the prototype.
- **Testing:**
    - Unit tests for the `parser.rs` module to ensure correct parsing of various valid and invalid bundle file scenarios.
    - Unit tests for the `bundler.rs` module to ensure correct file/directory creation, path handling, and collision detection logic.
    - Integration-style tests (CLI tests) to verify the overall behavior of `sprout` with sample bundle files and output directory states.
- **Naming Conventions:** Adhere to standard Rust naming conventions:
    - `snake_case` for functions, methods, variables, and modules.
    - `PascalCase` (or `CamelCase`) for types, structs, enums, and traits.
    - `UPPER_SNAKE_CASE` for constants.
- **Binary Size:** Strive for a reasonably small binary size for the release executable by configuring the release profile in `Cargo.toml` (e.g., `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`) and/or using `strip` utility post-compilation.
- **Comments:** Write clear comments for complex logic, public API functions, and any non-obvious decisions. Doc comments (`///`) for public items are encouraged.
- **Modularity:** Keep functions small and focused. Modules should have clear responsibilities.
