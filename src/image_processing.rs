use ab_glyph::{FontRef, PxScale};
use anyhow::Result;
use image::{DynamicImage, Rgb};
use imageproc::drawing::draw_text_mut;

const FONT_SIZE_FACTOR: u32 = 40;
const WIDTH_POSITION_FACTOR: u32 = 6;
const HEIGHT_POSITION_FACTOR: u32 = 16;

pub fn add_timestamp_to_image(img: &DynamicImage, timestamp: &str) -> Result<DynamicImage> {
    if timestamp.is_empty() {
        return Err(anyhow::anyhow!("Timestamp cannot be empty"));
    }

    let font = load_default_font()?;

    let mut img = img.to_rgb8();
    let (img_width, img_height) = img.dimensions();

    let font_size = calculate_font_size(img_width, img_height);
    let (text_x, text_y) = calculate_text_position(img_width, img_height);

    let color = Rgb([255u8, 165u8, 0u8]);

    draw_timestamp(&mut img, color, text_x, text_y, &font, font_size, timestamp);

    Ok(DynamicImage::ImageRgb8(img))
}

fn draw_timestamp(
    img: &mut image::RgbImage,
    color: Rgb<u8>,
    text_x: u32,
    text_y: u32,
    font: &FontRef<'static>,
    font_size: u32,
    timestamp: &str,
) {
    let scale = PxScale::from(font_size as f32);

    draw_text_mut(
        img,
        color,
        text_x as i32,
        text_y as i32,
        scale,
        font,
        timestamp,
    );
}

fn load_default_font() -> Result<FontRef<'static>> {
    let font_data = include_bytes!("../assets/digital-7.ttf");
    let font = FontRef::try_from_slice(font_data)
        .map_err(|e| anyhow::anyhow!("Failed to load default font: {}", e))?;
    Ok(font)
}

fn calculate_font_size(width: u32, height: u32) -> u32 {
    std::cmp::min(width / FONT_SIZE_FACTOR, height)
}

fn calculate_text_position(width: u32, height: u32) -> (u32, u32) {
    let x = width - (width / WIDTH_POSITION_FACTOR);
    let y = height - (height / HEIGHT_POSITION_FACTOR);
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add_timestamp_to_image_tests {
        use super::*;
        use image::RgbImage;

        #[test]
        fn test_add_timestamp_to_image_valid() {
            let width = 500;
            let height = 300;
            let rgb_img = RgbImage::new(width, height);
            let dynamic_img = DynamicImage::ImageRgb8(rgb_img);

            let timestamp = "2025-15-06 11:08:00";
            let result = add_timestamp_to_image(&dynamic_img, timestamp);

            assert!(result.is_ok(), "Failed to add timestamp to image");

            let image = result.unwrap();

            assert_eq!(image.width(), width);
            assert_eq!(image.height(), height);

            assert_ne!(
                image.to_rgb8(),
                dynamic_img.to_rgb8(),
                "Image should be modified after adding timestamp"
            );
        }

        #[test]
        fn test_add_timestamp_to_image_empty_timestamp_fails() {
            let width = 300;
            let height = 300;
            let rgb_img = RgbImage::new(width, height);
            let dynamic_img = DynamicImage::ImageRgb8(rgb_img);

            let result = add_timestamp_to_image(&dynamic_img, "");

            assert!(result.is_err(), "Expected error for empty timestamp");
        }
    }

    mod draw_timestamp_tests {
        use super::*;
        use image::{ImageBuffer, Rgb};

        #[test]
        fn test_draw_timestamp_applies_text() {
            let width = 400;
            let height = 300;
            let mut img = ImageBuffer::new(width, height);

            let color = Rgb([255, 165, 0]);
            let text_x = 300;
            let text_y = 200;
            let font = load_default_font().unwrap();
            let font_size = 20;
            let timestamp = "2023-01-01 12:34:56";

            // Take snapshot of pixels before
            let pixels_before = img.clone();

            // Draw timestamp
            draw_timestamp(&mut img, color, text_x, text_y, &font, font_size, timestamp);

            // Verify image was modified
            assert_ne!(
                pixels_before,
                img,
                "Image should be modified after drawing timestamp"
            );
        }
    }

    mod calculate_font_size_tests {
        use super::*;

        #[test]
        fn test_calculate_font_size_horizontal_valid() {
            assert_eq!(calculate_font_size(1920, 1080), 1920 / FONT_SIZE_FACTOR);
        }

        #[test]
        fn test_calculate_font_size_vertical_valid() {
            assert_eq!(calculate_font_size(1080, 1920), 1080 / FONT_SIZE_FACTOR);
        }

        #[test]
        fn test_calculate_font_size_limit_by_height() {
            // When height is smaller than width/40
            assert_eq!(calculate_font_size(4000, 50), 50);
        }
    }

    mod calculate_text_position_tests {
        use super::*;

        #[test]
        fn test_calculate_text_position_horizontal_valid() {
            assert_eq!(
                calculate_text_position(1920, 1080),
                (
                    1920 - 1920 / WIDTH_POSITION_FACTOR,
                    1080 - 1080 / HEIGHT_POSITION_FACTOR
                )
            );
        }

        #[test]
        fn test_calculate_text_position_vertical_valid() {
            assert_eq!(
                calculate_text_position(1080, 1920),
                (
                    1080 - 1080 / WIDTH_POSITION_FACTOR,
                    1920 - 1920 / HEIGHT_POSITION_FACTOR
                )
            );
        }
    }

    mod load_default_font_tests {
        use ab_glyph::Font;

        use super::*;

        #[test]
        fn test_load_default_font_loads_font() {
            let result = load_default_font();

            assert!(result.is_ok(), "Failed to load default font");

            let font = result.unwrap();

            assert!(
                font.glyph_id('A').0 > 0,
                "Font should contain basic characters"
            );

            assert!(
                font.glyph_id('0').0 > 0,
                "Font should contain basic characters"
            );
        }
    }

}
