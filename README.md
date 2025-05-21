# codesprout 🌱

![License](https://img.shields.io/github/license/nightconcept/codesprout)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nightconcept/codesprout/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/nightconcept/codesprout/badge.svg?branch=main)](https://coveralls.io/github/nightconcept/codesprout?branch=main)
![GitHub last commit](https://img.shields.io/github/last-commit/nightconcept/codesprout)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/nightconcept/codesprout/badge)](https://scorecard.dev/viewer/?uri=github.com/nightconcept/codesprout)

## 🌟 Overview

`codesprout` is a command-line interface that takes a bundle created by [gitingest](https://gitingest.com/) and sprouts it in the target directory.

`codesprout` is written in Rust for fun and learning.

## 🚀 Getting Started

### Prerequisites

*   **Rust:** Ensure you have Rust installed. You can get it from [rust-lang.org](https://www.rust-lang.org/). `codesprout` is built with the latest stable Rust version.

### Building `codesprout`

1.  **Clone the repository (if you haven't already):**
    ```bash
    git clone <repository-url>
    cd codesprout
    ```
2.  **Build for debugging:**
    ```bash
    cargo build
    ```
    The executable will be located at `target/debug/sprout`.

3.  **Build for release (optimized):**
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/sprout`.

## 🛠️ Usage

The `sprout` CLI tool takes a bundle file as input and creates the files and directories in a specified output location.

### Command Syntax:

```bash
sprout [BUNDLE_FILE_PATH] [OUTPUT_DIRECTORY_PATH]
```

Or using flags:

```bash
sprout --input <BUNDLE_FILE_PATH> --output <OUTPUT_DIRECTORY_PATH>
```

### Arguments & Options:

*   `BUNDLE_FILE_PATH`: (Positional or via `-i`/`--input`) Path to the bundle file. This is **required**.
*   `OUTPUT_DIRECTORY_PATH`: (Positional or via `-o`/`--output`) Path to the directory where files will be sprouted.
    *   Defaults to the current working directory if not specified.

## 릴리스 프로세스 (Release Process) 📦

This project uses `release-please-action` to automate releases. When commits adhering to the [Conventional Commits](https://www.conventionalcommits.org/) specification are merged into the `main` branch, `release-please` will:

1.  **Determine the next semantic version** based on the commit messages (e.g., `fix:` triggers a patch, `feat:` triggers a minor).
2.  **Generate a changelog** from these commit messages.
3.  **Create a pull request** (or directly create a release, depending on configuration) proposing these changes.
    *   If a pull request is created, it will update `Cargo.toml` with the new version and include the generated changelog.
    *   Merging this pull request will trigger the actual release.
4.  **Create a Git tag** for the new version (e.g., `v0.2.0`).
5.  **Publish a GitHub Release** with the generated changelog and compiled binaries for Linux, macOS (x86_64 and aarch64), and Windows.

### How to Trigger a Release:

1.  Ensure your commit messages on your feature branch follow the [Conventional Commits](https://www.conventionalcommits.org/) format.
    *   Example: `feat: Add new sprouting capability`
    *   Example: `fix: Correct parsing error for empty files`
    *   Example: `docs: Update usage instructions`
    *   Example: `chore: Refactor internal logging`
2.  Merge your pull request into the `main` branch.
3.  The `Release Please` GitHub Action will then run. If it determines a new version is warranted, it will either create a "Release PR" or directly create the release.
4.  If a "Release PR" is created:
    *   Review the PR (it will contain `Cargo.toml` version bumps and a `CHANGELOG.md` update).
    *   Merge the Release PR. This will trigger the GitHub Action to tag the release and upload assets.
5.  The new release will appear on the [GitHub Releases page](https://github.com/nightconcept/codesprout/releases) with attached binaries.

## 🧪 Testing

This project uses Rust's built-in testing framework.

*   **Run all tests:**
    ```bash
    cargo test
    ```
*   **Run tests with verbose output:**
    ```bash
    cargo test -- --nocapture
    ```
*   **Check code coverage (requires `cargo-tarpaulin`):**
    ```bash
    cargo tarpaulin --verbose --all-features --workspace --engine Llvm --out Xml --output-dir target/tarpaulin
    ```
    Coverage reports are uploaded to Coveralls on pushes to `main`.

## � Future Ideas

While the current prototype is focused and functional, here are some ideas for future enhancements:

*   **Reverse Operation ("Bundling"):** Create a bundle file from an existing directory.
*   **Overwrite Options:** Add flags like `--force` to allow overwriting files.

## License

This project is licensed under the MIT License. See [LICENSE](docs/LICENSE) for details.
