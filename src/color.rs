use crate::utility::get_value_around;

const MAX_SIMILIARITY_OFFSET: u8 = 10;

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn hex(hex: u32) -> Color {
        let r = (hex & 0xFF0000) >> 16;
        let g = (hex & 0x00FF00) >> 8;
        let b = hex & 0x0000FF;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: 255,
        }
    }

    /// Returns a color similiar to this one
    pub fn similiar(&self) -> Color {
        let Color { r, g, b, a } = *self;

        let new_r = get_value_around(r as i32, MAX_SIMILIARITY_OFFSET as i32).clamp(0, 255) as u8;
        let new_g = get_value_around(g as i32, MAX_SIMILIARITY_OFFSET as i32).clamp(0, 255) as u8;
        let new_b = get_value_around(b as i32, MAX_SIMILIARITY_OFFSET as i32).clamp(0, 255) as u8;

        Color::rgba(new_r, new_g, new_b, a)
    }
}
