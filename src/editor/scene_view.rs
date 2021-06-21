use super::*;

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:     Scene,
    viewport:  Viewport,
    show_grid: bool,
}

impl SceneView {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::new();
        scene.make_rects();

        Self {
            scene,
            viewport: Viewport::new(ctx),
            show_grid: true,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard) {
        let g = keyboard.is_pressed(KeyCode::G);

        if g {
            self.show_grid = !self.show_grid;
        }

        self.viewport.handle_keys(keyboard);
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.viewport.set_size(ctx);

        let click = ggez::input::mouse::button_pressed(ctx, MouseButton::Left);
        if click {
            let pos = ggez::input::mouse::position(ctx);
            let origin = self.viewport.origin();

            let position = Point::from([pos.x - origin.x, pos.y - origin.y]);
            let scale = self.viewport.scale();
            let tile = Point::from([
                (position.x / (scale * TILE_WIDTH)).floor() as i16,
                (position.y / (scale * TILE_HEIGHT)).floor() as i16,
            ]);
            self.scene.floors.insert(tile, Floor::Floor);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer) {
        self.scene.render(tile_renderer);
        tile_renderer.draw(ctx, self.viewport.origin(), self.viewport.scale());

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }
}
