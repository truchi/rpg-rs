use super::*;

#[derive(Clone, Default, Debug)]
pub struct TilesView {}

impl View for TilesView {
    fn new(ctx: &mut Context) -> Self {
        Self {}
    }

    fn events(&mut self, keyboard: &Keyboard) {}

    fn update(&mut self, ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer) {
        let mut mesh = MeshBuilder::new();

        let scale = 2.;
        let width = scale * TILE_WIDTH;
        let height = scale * TILE_HEIGHT;
        let margin_x = TILE_WIDTH;
        let margin_y = TILE_HEIGHT;

        for (i, floor) in Floor::all().iter().enumerate() {
            let i = i as f32;
            let x = margin_x + i * (width + margin_x);
            let y = margin_y;

            mesh.rectangle(
                DrawMode::stroke(2.),
                [x - 1., y - 1., width + 2., height + 2.].into(),
                Color::new(0., 1., 0., 1.),
            )
            .unwrap();
            tile_renderer.add_raw(floor.tile(), [x, y], scale);
        }

        tile_renderer.draw(ctx, [0., 0.], 1.);
        mesh.build(ctx)
            .unwrap()
            .draw(ctx, DrawParam::new().dest([0., 0.]))
            .unwrap();
    }
}
