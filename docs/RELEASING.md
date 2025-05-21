# Releasing `codesprout`

This document outlines the process for creating official releases and pre-releases for the `codesprout` project.

## Automated Release Process Overview

This project uses `release-please-action` to automate releases. When commits adhering to the [Conventional Commits](https://www.conventionalcommits.org/) specification are merged into the `main` branch, `release-please` will:

1.  **Determine the next semantic version** based on the commit messages (e.g., `fix:` triggers a patch, `feat:` triggers a minor).
2.  **Generate a changelog** from these commit messages.
3.  **Create a pull request** (or directly create a release, depending on configuration) proposing these changes.
    *   If a pull request is created, it will update `Cargo.toml` with the new version and include the generated changelog.
    *   Merging this pull request will trigger the actual release.
4.  **Create a Git tag** for the new version (e.g., `v0.2.0`).
5.  **Publish a GitHub Release** with the generated changelog and compiled binaries for Linux, macOS (x86_64 and aarch64), and Windows.

This process is documented in the main [README.md](../README.md#릴리스-프로세스-release-process-📦).

## Creating Pre-Releases (e.g., Alpha, Beta)

To create a specific pre-release version (e.g., `0.1.0-alpha.1`), you need to use a special commit message footer.

### Steps:

1.  **Prepare Your Code:**
    *   Ensure all code changes intended for the pre-release are committed to your local `main` branch (or the feature branch you will merge into `main`).

2.  **Craft a Special Commit:**
    *   Create a new commit (or amend an existing one if it's the last commit and not yet pushed).
    *   The commit message **must** follow the [Conventional Commits](https://www.conventionalcommits.org/) standard.
    *   Include a `Release-As:` footer in the commit message body, specifying the exact pre-release version.

    **Example Commit Message for `0.1.0-alpha.1`:**
    ```
    feat: Implement initial features for alpha release

    This commit includes the core functionality planned for the first alpha
    version of 0.1.0. It's ready for preliminary testing.

    Release-As: 0.1.0-alpha.1
    ```
    *   The `feat:` prefix (or `fix:`, `chore:`, etc.) is important for `release-please` to categorize the changes in the changelog.
    *   The `Release-As: 0.1.0-alpha.1` line explicitly tells `release-please` to create this specific version, overriding its normal version calculation for this instance.

3.  **Push to `main`:**
    *   Push this commit to the `main` branch on GitHub.
    ```bash
    git push origin main
    ```

4.  **`release-please` Takes Over:**
    *   The push to `main` will trigger the [`.github/workflows/release.yml`](../.github/workflows/release.yml) workflow.
    *   `release-please-action` will read the commit history and detect the `Release-As:` footer.
    *   It will then proceed to:
        *   Update the version in `Cargo.toml` to the specified pre-release version (e.g., `0.1.0-alpha.1`).
        *   Generate or update a `CHANGELOG.md` file.
        *   Commit these file changes directly to the `main` branch.
        *   Create a Git tag (e.g., `v0.1.0-alpha.1`).
        *   Create a GitHub Release with the corresponding title and changelog notes.
        *   The `build-and-upload-assets` job in the workflow will then build the binaries and upload them to this GitHub Release.

### Important Considerations for Pre-Releases:

*   **Base Version in `Cargo.toml`:** `release-please` uses the version in your `Cargo.toml` as a starting point. Ensure it's at a suitable base (e.g., `0.1.0` if you're creating `0.1.0-alpha.1`) before `release-please` runs.
*   **Subsequent Pre-Releases:** To release `0.1.0-alpha.2`, make further commits and then create another commit with the footer `Release-As: 0.1.0-alpha.2`.
*   **Exiting Pre-Release to Stable:** When you're ready to release the stable version (e.g., `0.1.0` after a series of alphas/betas), make a commit with the footer `Release-As: 0.1.0`. For example:
    ```
    feat: Finalize features for 0.1.0

    All planned features for the 0.1.0 release are complete and tested.
    This version is ready for stable release.

    Release-As: 0.1.0
    ```

This `Release-As` footer provides precise control over the version number, which is essential for managing pre-release cycles.
