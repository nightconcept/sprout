# PowerShell script to build the 'sprout' project in release mode
# and install it to the user's local bin directory.

# Strict mode
Set-StrictMode -Version Latest

# Stop on first error
$ErrorActionPreference = "Stop"

Write-Host "Starting install script for sprout..."

# 1. Define project root (assuming script is run from project root or scripts/ directory)
$ProjectRoot = $PSScriptRoot | Split-Path
if ($PSScriptRoot -like "$($ProjectRoot)\scripts") {
    # Script is in scripts/, so ProjectRoot is one level up
    # No action needed, $ProjectRoot is correct
} elseif ($PSScriptRoot -eq $ProjectRoot) {
    # Script is in project root
    # No action needed, $ProjectRoot is correct
} else {
    Write-Warning "Script is not in the expected 'scripts/' subdirectory or project root. Assuming current directory is project root."
    $ProjectRoot = Get-Location
}

Write-Host "Project root identified as: $ProjectRoot"

# 2. Build the project in release mode
Write-Host "Building 'sprout' in release mode... (cargo build --release)"
Push-Location $ProjectRoot
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Cargo build failed with exit code $LASTEXITCODE."
        exit $LASTEXITCODE
    }
    Write-Host "Build successful."
}
catch {
    Write-Error "An error occurred during the build process: $($_.Exception.Message)"
    exit 1
}
finally {
    Pop-Location
}

# 3. Define source and destination paths
$ExecutableName = "sprout.exe" # Assuming Windows, adjust if cross-platform build is intended here
$SourceFile = Join-Path -Path $ProjectRoot -ChildPath "target\release\$ExecutableName"
$DestinationDir = Join-Path -Path $env:USERPROFILE -ChildPath ".local\bin"

Write-Host "Source executable: $SourceFile"
Write-Host "Target directory: $DestinationDir"

# 4. Check if the source file exists
if (-not (Test-Path $SourceFile -PathType Leaf)) {
    Write-Error "Build artifact '$SourceFile' not found. Build might have failed or produced a different output."
    exit 1
}

# 5. Ensure the destination directory exists
if (-not (Test-Path $DestinationDir -PathType Container)) {
    Write-Host "Destination directory '$DestinationDir' does not exist. Creating it..."
    try {
        New-Item -ItemType Directory -Force -Path $DestinationDir | Out-Null
        Write-Host "Destination directory created."
    }
    catch {
        Write-Error "Failed to create destination directory '$DestinationDir': $($_.Exception.Message)"
        exit 1
    }
}

# 6. Copy the executable
Write-Host "Copying '$ExecutableName' to '$DestinationDir'..."
try {
    Copy-Item -Path $SourceFile -Destination $DestinationDir -Force
    Write-Host "'$ExecutableName' successfully copied to '$DestinationDir'."
}
catch {
    Write-Error "Failed to copy '$ExecutableName' to '$DestinationDir': $($_.Exception.Message)"
    exit 1
}

Write-Host "Installation complete. You can now run '$ExecutableName' from your terminal if '$DestinationDir' is in your PATH."