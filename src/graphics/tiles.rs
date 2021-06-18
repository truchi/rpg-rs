use super::*;

#[derive(Clone, Debug)]
pub struct Tiles {
    batch: SpriteBatch,
}

impl Tiles {
    pub fn new(ctx: &mut Context) -> Self {
        let mut image = Image::new(ctx, ARTPACK).expect("Cannot read artpack");
        image.set_filter(FilterMode::Nearest);

        Self {
            batch: SpriteBatch::new(image),
        }
    }

    pub fn add(&mut self, ctx: &mut Context, tile: Tile, pos: impl Into<Point>) {
        let pos = pos.into();

        self.batch.add(
            DrawParam::new()
                .src(tile.rect())
                .dest([pos.x * TILE_WIDTH, pos.y * TILE_HEIGHT]),
        );
    }

    pub fn draw(&mut self, ctx: &mut Context, scale: f32) -> &mut Self {
        self.batch
            .draw(ctx, DrawParam::new().scale([scale, scale]))
            .unwrap();
        self
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }

    pub fn render(&mut self, ctx: &mut Context, tiles: impl Iterator<Item = (Tile, Point)>) {
        for (tile, pos) in tiles {
            self.add(ctx, tile, pos);
        }

        self.draw(ctx, 2.);
        self.clear();
    }
}
