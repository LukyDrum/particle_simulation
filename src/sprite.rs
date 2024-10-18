use std::collections::HashSet;

use image::{open, ImageError};

pub struct Sprite {
    pub pixels: Vec<u32>,
    pub width: u32,
    pub height: u32,
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

    /// Returns a HashSet off all the unique colors in the sprite
    pub fn get_unique_colors(&self) -> HashSet<u32> {
        HashSet::from_iter(self.pixels.clone())
    }
}
