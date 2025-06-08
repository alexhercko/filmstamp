mod image_metadata;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image_metadata::extract_timestamp_from_image;

#[derive(Parser)]
#[command(name = "filmstamp", version, about = "Add film-like timestamps to photos using Exif data.", long_about = None)]
struct CliArgs {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    println!("Processing file: {}", args.input.display());

    match extract_timestamp_from_image(&args.input) {
        Ok(timestamp) => println!("Extracted timestamp: {}", timestamp),
        Err(e) => {
            eprintln!("Error extracting timestamp from EXIF data: {}", e);
            return Err(e);
        }
    };

    Ok(())
}
