use crate::Offset;

pub const DEFAULT_VELOCITY: f32 = 1.0;
pub const MAX_VELOCITY: f32 = 5.0;
pub const GRAVITY: f32 = 0.1;
pub const SWITCH_SLOWDOWN: f32 = 0.1;
pub const MAX_DENSITY: u8 = 255;
pub const MAX_GAS_DENSITY: u8 = 64;
pub const FIRE_COLOR: u32 = 0x940C0C;
pub const CELL_DEFAULT_PRESSURE: i32 = 0;
pub const CELL_PRESSURE_DIFF: i32 = 5;

pub const UP: Offset = Offset { x: 0, y: -1 };
pub const DOWN: Offset = Offset { x: 0, y: 1 };
pub const LEFT: Offset = Offset { x: -1, y: 0 };
pub const RIGHT: Offset = Offset { x: 1, y: 0 };
