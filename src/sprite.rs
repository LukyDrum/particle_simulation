use image::{open, ImageError};

pub struct Sprite {
    pixels: Vec<u32>,
    width: u32,
    height: u32,
}

impl Sprite {
    pub fn load(img_path: &str) -> Result<Sprite, ImageError> {
        // Read the image, possibly return error
        let img_buffer = open(img_path)?.into_rgb8();
        // Get dimensions
        let (width, height) = img_buffer.dimensions();
        // Convert pixels RGB values to u32
        let pixels: Vec<u32> = img_buffer
            .pixels()
            .map(|rgb| {
                let mut color: u32 = 0;
                // Match colors
                let [r, g, b] = rgb.0;
                // Set max opacity
                color += 0xFF << 24;
                // Set colors
                color += (r as u32) << 16;
                color += (g as u32) << 8;
                color += b as u32;

                color
            })
            .collect();

        Ok(Sprite {
            pixels,
            width,
            height,
        })
    }
}
