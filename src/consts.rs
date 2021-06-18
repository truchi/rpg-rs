use super::*;

pub const MAX_ZOOM: u8 = 7; // MUST NOT be > 7
pub const DEBUG_HITBOXES: bool = true;
pub const KEYBOARD_DEBOUNCE: Duration = Duration::from_millis(100);
pub const ARTPACK: &'static str = "/tiles.png";
pub const ARTPACK_WIDTH: f32 = 512.;
pub const ARTPACK_HEIGHT: f32 = 512.;
pub const TILE_WIDTH: f32 = 16.;
pub const TILE_HEIGHT: f32 = 16.;
pub const FPS: f32 = 30.;
pub const RATE: f32 = 1.0 / FPS;
pub const DEADZONE: f32 = 0.11;
