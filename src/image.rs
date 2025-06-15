use anyhow::Result;
use image::{DynamicImage, ImageReader};

pub fn load_image(path: &std::path::Path) -> Result<DynamicImage> {
    let image = ImageReader::open(path)
        .map_err(|e| anyhow::anyhow!("Failed to open image: {}", e))?
        // Determine the image format based on the context
        .with_guessed_format()
        .map_err(|e| anyhow::anyhow!("Failed to guess image format: {}", e))?
        // Construct a reader using the guessed format
        .decode()
        .map_err(|e| anyhow::anyhow!("Failed to read image: {}", e))?;
    Ok(image)
}

pub fn save_image(image: &DynamicImage, path: &std::path::Path) -> Result<()> {
    image
        .save(path)
        .map_err(|e| anyhow::anyhow!("Failed to save image: {}", e))?;
    Ok(())
}
