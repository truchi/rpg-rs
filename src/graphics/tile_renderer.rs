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

    pub fn add(&mut self, tile: Tile, pos: impl Into<Point>) {
        let pos = pos.into();

        self.batch.add(
            DrawParam::new()
                .src(tile.rect())
                .dest([pos.x * TILE_WIDTH, pos.y * TILE_HEIGHT]),
        );
    }

    pub fn draw(&mut self, ctx: &mut Context, viewport: Viewport) -> &mut Self {
        let origin = viewport.origin();
        let scale = viewport.scale();
        let param = DrawParam::new().dest(origin).scale([scale, scale]);

        self.batch.draw(ctx, param).unwrap();
        self
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
