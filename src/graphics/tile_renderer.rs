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

    pub fn add(&mut self, params: impl Params) {
        self.batch.add(params.params());
    }

    pub fn draw(&mut self, ctx: &mut Context, origin: impl Into<Point>, scale: f32) {
        let param = DrawParam::new().dest(origin).scale([scale, scale]);

        self.batch.draw(ctx, param).unwrap();
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
