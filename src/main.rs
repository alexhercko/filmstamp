mod image;
mod image_metadata;
mod image_processing;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use image::{load_image, save_image};
use image_metadata::extract_timestamp_from_exif;
use image_processing::add_timestamp_to_image;

#[derive(Parser)]
#[command(name = "filmstamp", version, about = "Add film-like timestamps to photos using Exif data.", long_about = None)]
struct CliArgs {
    input: PathBuf,

    #[arg(short, long, help = "Output file path.")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    println!("Processing file: {}", args.input.display());

    let (img, exif) = load_image(&args.input)
        .with_context(|| format!("Failed to load image: {}", args.input.display()))?;

    let exif = match exif {
        Some(data) => data,
        None => {
            return Err(anyhow::anyhow!(
                "No EXIF metadata found in the image: {}",
                args.input.display()
            ));
        }
    };

    let timestamp = extract_timestamp_from_exif(exif).with_context(|| {
        format!(
            "Error extracting timestamp from EXIF data of image: {}",
            args.input.display()
        )
    })?;

    // Format specifiers: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    let timestamp = timestamp.format("%-d  %-m  %-Y   %-H:%-M").to_string();

    let stamped_img = add_timestamp_to_image(&img, &timestamp)
        .with_context(|| format!("Error adding timestamp to image: {}", args.input.display()))?;

    save_image(&stamped_img, &args.output)
        .with_context(|| format!("Failed to save stamped image to: {}", args.output.display()))?;

    println!("Image saved to: {}", args.output.display());

    Ok(())
}
