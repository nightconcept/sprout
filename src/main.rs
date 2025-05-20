use clap::Parser;
use std::path::PathBuf;

/// sprout - A CLI tool to sprout files from a bundle.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, infer_long_args = true)]
struct CliArgs {
    /// Path to the bundle file (positional).
    /// Required unless -i/--input is used.
    #[arg(name = "BUNDLE_FILE_PATH", required_unless_present = "input")]
    bundle_file_path: Option<PathBuf>,

    /// Output directory path (positional).
    /// Defaults to the current directory if not specified and -o/--output is not used.
    #[arg(name = "OUTPUT_DIRECTORY_PATH", default_value = ".")]
    output_directory_path: PathBuf,

    /// Specify bundle file path via flag (alternative to positional BUNDLE_FILE_PATH).
    #[arg(short, long, value_name = "PATH", conflicts_with = "BUNDLE_FILE_PATH")]
    input: Option<PathBuf>,

    /// Specify output directory path via flag (overrides positional OUTPUT_DIRECTORY_PATH).
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    // Determine effective input path.
    // Clap ensures that either `bundle_file_path` or `input` is Some, but not both,
    // and that at least one of them is provided.
    let bundle_path = match (args.bundle_file_path, args.input) {
        (Some(p), None) => p,
        (None, Some(i)) => i,
        // The following cases should be prevented by clap's validation:
        // (None, None) => Error: missing required input (handled by `required_unless_present`)
        // (Some(_), Some(_)) => Error: conflicting arguments (handled by `conflicts_with`)
        _ => unreachable!("Clap should ensure one input source is exclusively provided and valid."),
    };

    // Determine effective output path.
    // If -o/--output is provided, it takes precedence.
    // Otherwise, use the positional `output_directory_path` (which defaults to ".").
    let final_output_path = if let Some(output_flag_path) = args.output {
        output_flag_path
    } else {
        args.output_directory_path
    };

    println!("Effective bundle file path: {:?}", bundle_path);
    println!("Effective output directory path: {:?}", final_output_path);

    // Placeholder for further processing based on Task 1.3
    // 1. Resolve final input and output paths (done above)
    // 2. (Stub) Call parser::process_bundle_file(&bundle_path)
    // 3. (Stub) If parsing/validation successful, call bundler::create_files(parsed_data, &final_output_path)

    Ok(())
}
