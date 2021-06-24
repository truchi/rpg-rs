use super::*;

pub const SCALE_LIMIT: f32 = 10.;
pub const SCROLL_SENSITIVITY: Point = Point { x: 30., y: -30. };
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
