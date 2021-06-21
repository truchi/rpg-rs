use super::*;

mod grid;
mod tile;
mod tile_renderer;

pub use grid::*;
pub use tile::*;
pub use tile_renderer::*;

pub trait Params {
    fn params(self) -> DrawParam;
}

impl<T: Into<Point>> Params for (Tile, T) {
    fn params(self) -> DrawParam {
        let Point { x, y } = self.1.into();

        DrawParam::new()
            .src(self.0.rect())
            .dest([x * TILE_WIDTH, y * TILE_HEIGHT])
    }
}

impl<T: Into<Point>> Params for (Tile, T, f32) {
    fn params(self) -> DrawParam {
        (self.0, self.1).params().scale([self.2, self.2])
    }
}

impl<T: Into<Point>> Params for (Tile, T, Orientation) {
    fn params(self) -> DrawParam {
        // (self.0, self.1).params() // TODO
        let Point { x, y } = self.1.into();

        let (dest, rotation) = match self.2 {
            North => ([x, y], 0.),
            East => ([-y, x], TAU / 4.),
            South => ([-x, -y], TAU / 2.),
            West => ([y, -x], -TAU / 4.),
        };

        (self.0, dest).params().rotation(rotation)
    }
}
