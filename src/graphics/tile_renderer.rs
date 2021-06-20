use super::*;

#[derive(Clone, Debug)]
pub struct TileRenderer {
    batch: SpriteBatch,
}

impl TileRenderer {
    pub fn new(ctx: &mut Context) -> Self {
        let mut image = Image::new(ctx, ARTPACK).expect("Cannot read artpack");
        image.set_filter(FilterMode::Nearest);

        Self {
            batch: SpriteBatch::new(image),
        }
    }

    // TODO replace uses with add_raw?
    pub fn add(&mut self, tile: Tile, pos: impl Into<Point>) {
        let pos = pos.into();

        self.batch.add(
            DrawParam::new()
                .src(tile.rect())
                .dest([pos.x * TILE_WIDTH, pos.y * TILE_HEIGHT]),
        );
    }

    pub fn add_raw(&mut self, tile: Tile, pos: impl Into<Point>, scale: f32) {
        self.batch.add(
            DrawParam::new()
                .src(tile.rect())
                .dest(pos)
                .scale([scale, scale]),
        );
    }

    pub fn draw(&mut self, ctx: &mut Context, origin: impl Into<Point>, scale: f32) {
        let param = DrawParam::new().dest(origin).scale([scale, scale]);

        self.batch.draw(ctx, param).unwrap();
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
