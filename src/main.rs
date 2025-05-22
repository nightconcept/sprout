use clap::Parser;
use std::path::PathBuf;

mod bundler;
mod parser;

/// sprout - A CLI tool to sprout files from a bundle.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, infer_long_args = true, arg_required_else_help = true)]
struct CliArgs {
    /// Positional arguments: <BUNDLE_FILE_PATH> [OUTPUT_DIRECTORY_PATH]
    /// BUNDLE_FILE_PATH is required if -i/--input is not used.
    /// OUTPUT_DIRECTORY_PATH defaults to '.' if not specified.
    #[arg(name = "PATHS", num_args = 0..=2)]
    paths: Vec<PathBuf>,

    /// Specify bundle file path via flag (alternative to positional BUNDLE_FILE_PATH).
    #[arg(short, long, value_name = "PATH")]
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

    let bundle_path = if let Some(ref input_flag_path) = args.input {
        input_flag_path.clone()
    } else {
        // If -i is not used, the first positional arg must be the bundle path
        if args.paths.is_empty() {
            // This case should ideally be caught by clap if we could make the first
            // positional arg conditionally required. Since that's tricky,
            // we handle it manually.
            // A more robust solution might involve custom validation or rethinking arg structure
            // if this manual check becomes too complex.
            return Err(anyhow::anyhow!(
                "Missing bundle file path. Provide it as the first positional argument or use -i/--input."
            ));
        }
        args.paths[0].clone()
    };

    let final_output_path = if let Some(output_flag_path) = args.output {
        output_flag_path
    } else {
        // If -o is not used, determine output path from positional args
        if args.input.is_some() {
            // If -i was used, the first positional arg (if any) is the output path
            args.paths
                .first()
                .cloned()
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            // If -i was NOT used, the bundle path was paths[0].
            // So, the output path is paths[1] if present, otherwise default.
            args.paths
                .get(1)
                .cloned()
                .unwrap_or_else(|| PathBuf::from("."))
        }
    };

    // Ensure bundle_path is not misinterpreted as output_path if it's the same as default
    if bundle_path == PathBuf::from(".")
        && final_output_path == PathBuf::from(".")
        && args.input.is_none()
        && args.paths.len() == 1
    {
        // This means only one positional arg was given (the bundle path), and it resolved to "."
        // and no -o was given, so output defaults to ".". This is correct.
        // No action needed, but good to be aware of this edge case.
    }

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
