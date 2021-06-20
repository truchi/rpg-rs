use super::*;

#[derive(Clone, Default, Debug)]
pub struct TilesView {}

impl TilesView {}

impl View for TilesView {
    fn new(ctx: &mut Context) -> Self {
        Self {}
    }

    fn events(&mut self, keyboard: &Keyboard) {}

    fn update(&mut self, ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context, tiles: &mut Tiles) {}
}
