use anyhow::Result;
use image::{DynamicImage, ImageDecoder, ImageReader};

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

/// Saves the provided `DynamicImage` to the specified path.
pub fn save_image(image: &DynamicImage, path: &std::path::Path) -> Result<()> {
    image
        .save(path)
        .map_err(|e| anyhow::anyhow!("Failed to save image: {}", e))?;
    Ok(())
}
