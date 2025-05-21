use clap::Parser;
use std::path::PathBuf;

mod bundler;
mod parser;

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

    /// Force overwrite of existing files.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let bundle_path = match (args.bundle_file_path, args.input) {
        (Some(p), None) => p,
        (None, Some(i)) => i,
        _ => unreachable!("Clap should ensure one input source is exclusively provided and valid."),
    };

    let final_output_path = if let Some(output_flag_path) = args.output {
        output_flag_path
    } else {
        args.output_directory_path
    };

    let parsed_data = parser::parse_bundle(&bundle_path)?;

    if parsed_data.is_empty() {
        println!(
            "Bundle file '{}' is empty or contains no valid entries. Nothing to sprout.",
            bundle_path.display()
        );
        return Ok(());
    }

    if !args.force {
        bundler::check_for_collisions(&parsed_data, &final_output_path)?;
    }

    bundler::create_files_from_bundle(&parsed_data, &final_output_path, args.force)?;

    println!(
        "Successfully sprouted {} file(s) from '{}' to '{}'.{}",
        parsed_data.len(),
        bundle_path.display(),
        final_output_path.display(),
        if args.force {
            " (files overwritten if necessary)"
        } else {
            ""
        }
    );
    Ok(())
}
