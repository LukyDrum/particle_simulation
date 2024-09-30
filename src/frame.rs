#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameError {
    OutOfBounds,
}

pub struct Frame {
    pub logical_width: usize,
    pub logical_height: usize,
    pub logical_scale: usize,
    pub buffer: Vec<u32>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame {
            logical_width: width,
            logical_height: height,
            logical_scale: 1,
            buffer: vec![0; width * height],
        }
    }

    pub fn new_with_scale(
        logical_width: usize,
        logical_height: usize,
        logical_scale: usize,
    ) -> Frame {
        if logical_scale < 1 {
            panic!("Logical scale must larger than 1");
        }

        let width = logical_width * logical_scale;
        let height = logical_height * logical_scale;

        Frame {
            logical_width,
            logical_height,
            buffer: vec![0; width * height],
            logical_scale,
        }
    }

    /// Returns the real width of the frame buffer in pixels.
    pub fn width(&self) -> usize {
        self.logical_width * self.logical_scale
    }

    /// Returns the real height of the frame buffer in pixels.
    pub fn height(&self) -> usize {
        self.logical_height * self.logical_scale
    }

    /// Converts a position in pixels to a logical position.
    pub fn real_pos_to_logical(&self, x: usize, y: usize) -> (usize, usize) {
        let logical_x = (x / self.logical_scale) as usize;
        let logical_y = (y / self.logical_scale) as usize;

        (logical_x, logical_y)
    }

    /// Draws a logical pixel to the frame buffer.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), FrameError> {
        // Check constaints
        if x >= self.logical_width || y >= self.logical_height {
            return Err(FrameError::OutOfBounds);
        }

        // Top left corner of the rectangle
        let row = y * self.logical_scale;
        let col = x * self.logical_scale;

        // Draw the rectangle
        for row_offset in 0..self.logical_scale {
            let off_row = row + row_offset;

            for col_offset in 0..self.logical_scale {
                let off_col = col + col_offset;
                let index = off_row * self.width() + off_col;

                self.buffer[index] = color;
            }
        }

        Ok(())
    }
}
