use std::{fs::File, io::BufWriter};

use anyhow::Result;
use image::{codecs::jpeg::JpegEncoder, DynamicImage, ImageDecoder, ImageReader};


/// Loads an image from the specified path and returns a tuple containing the loaded `DynamicImage`
/// along with its EXIF metadata if present.
///
/// Returns an error if the image cannot be opened, the format cannot be guessed or the image
/// decoder fails to read the metadata or image data.
pub fn load_image(path: &std::path::Path) -> Result<(DynamicImage, Option<Vec<u8>>)> {
    let mut decoder = ImageReader::open(path)
        .map_err(|e| anyhow::anyhow!("Failed to open image: {}", e))?
        .with_guessed_format()
        .map_err(|e| anyhow::anyhow!("Failed to guess image format: {}", e))?
        .into_decoder()
        .map_err(|e| anyhow::anyhow!("Failed to open decoder for image: {}", e))?;

    let exif = decoder
        .exif_metadata()
        .map_err(|e| anyhow::anyhow!("Failed to read EXIF metadata: {}", e))?;

    let image = DynamicImage::from_decoder(decoder)
        .map_err(|e| anyhow::anyhow!("Failed to read image from decoder: {}", e))?;

    Ok((image, exif))
}

/// Saves the provided `DynamicImage` to the specified path with the specified JPEG quality.
pub fn save_image(image: &DynamicImage, path: &std::path::Path, jpeg_quality: u8) -> Result<()> {
    let output_file = File::create(path)
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;
    let writer = BufWriter::new(output_file);
    let encoder = JpegEncoder::new_with_quality(writer, jpeg_quality);
    image
        .write_with_encoder(encoder)
        .map_err(|e| anyhow::anyhow!("Failed to save image: {}", e))?;
    Ok(())
}
