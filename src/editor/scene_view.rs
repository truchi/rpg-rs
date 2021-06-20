use super::*;

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:     Scene,
    viewport:  Viewport,
    show_grid: bool,
}

impl SceneView {
    /*
    fn update(&mut self, ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut Context) {
    }

    fn handle_keys(&mut self) {
    }
    */
}

impl View for SceneView {
    fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::default();
        scene.make_rects();

        Self {
            scene,
            viewport: Viewport::new(ctx),
            show_grid: true,
        }
    }

    fn events(&mut self, keyboard: &Keyboard) {
        let g = keyboard.is_pressed(KeyCode::G);

        if g {
            self.show_grid = !self.show_grid;
        }

        self.viewport.handle_keys(keyboard);
    }

    fn update(&mut self, ctx: &mut Context) {
        self.viewport.set_size(ctx);

        // let pos = ggez::input::mouse::position(ctx);
        // let click = ggez::input::mouse::button_pressed(ctx,
        // MouseButton::Left); if click {
        // let position = Point::from([pos.x - self.origin.x, pos.y -
        // self.origin.y]); let scale = self.viewport.scale();
        // let tile = Point::from([
        // (position.x / (scale * TILE_WIDTH)).floor() as i16,
        // (position.y / (scale * TILE_HEIGHT)).floor() as i16,
        // ]);
        // self.scene.floors.insert(tile, Floor::Floor1);
        // }

        // Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, tiles: &mut Tiles) {
        self.scene.render(tiles);
        tiles.draw(ctx, self.viewport).clear();

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }
}
