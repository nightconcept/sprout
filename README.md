# sprout 🌱

![License](https://img.shields.io/github/license/nightconcept/sprout)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nightconcept/sprout/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/nightconcept/sprout/badge.svg?branch=main)](https://coveralls.io/github/nightconcept/sprout?branch=main)
![GitHub last commit](https://img.shields.io/github/last-commit/nightconcept/sprout)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/nightconcept/sprout/badge)](https://scorecard.dev/viewer/?uri=github.com/nightconcept/sprout)

## 🌟 Overview

`sprout` is a command-line interface that takes a bundle created by [gitingest](https://gitingest.com/) and sprouts it in the target directory.

`sprout` is written in Rust for fun and learning.

## 🚀 Getting Started

### Prerequisites

*   **Rust:** Ensure you have Rust installed. You can get it from [rust-lang.org](https://www.rust-lang.org/). `sprout` is built with the latest stable Rust version.

### Building `sprout`

1.  **Clone the repository (if you haven't already):**
    ```sh
    git clone https://github.com/nightconcept/sprout.git
    cd sprout
    ```
2.  **Build for debugging:**
    ```sh
    cargo build
    ```
    The executable will be located at `target/debug/sprout`.

3.  **Build for release (optimized):**
    ```sh
    cargo build --release
    ```
    The executable will be located at `target/release/sprout`.

## 🛠️ Usage

The `sprout` CLI tool takes a bundle file as input and creates the files and directories in a specified output location.

### Command Syntax:

```sh
sprout [OPTIONS] [BUNDLE_FILE_PATH] [OUTPUT_DIRECTORY_PATH]
```

Or using flags:

```sh
sprout --input <BUNDLE_FILE_PATH> --output <OUTPUT_DIRECTORY_PATH>
```

### Arguments & Options:

*   `BUNDLE_FILE_PATH`: (Positional or via `-i`/`--input`) Path to the bundle file. This is **required**.
*   `OUTPUT_DIRECTORY_PATH`: (Positional or via `-o`/`--output`) Path to the directory where files will be sprouted.
    *   Defaults to the current working directory if not specified.
*   `-f`, `--force`: (Optional) If specified, `sprout` will overwrite any existing files in the output directory that conflict with files from the bundle. Without this flag, `sprout` will abort if any collisions are detected.

## 📜 License

This project is licensed under the MIT License. See [LICENSE](docs/LICENSE) for details.
