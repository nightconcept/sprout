# codesprout 🌱

[![Coverage Status](https://coveralls.io/repos/github/nightconcept/codesprout/badge.svg?branch=main)](https://coveralls.io/github/nightconcept/codesprout?branch=main)

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

## 🔮 Future Ideas

While the current prototype is focused and functional, here are some ideas for future enhancements:

*   **Reverse Operation ("Bundling"):** Create a bundle file from an existing directory.
*   **Overwrite Options:** Add flags like `--force` to allow overwriting files.

## License
