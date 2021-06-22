use super::*;

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:           Scene,
    viewport:        Viewport,
    show_grid:       bool,
    floor_selection: Option<(Point<i16>, Point<i16>)>,
}

impl SceneView {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::new();
        scene.make_rects();

        Self {
            scene,
            viewport: Viewport::new(ctx),
            show_grid: false,
            floor_selection: None,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard) {
        self.viewport.handle_keys(keyboard);

        if keyboard.is_pressed(KeyCode::G) {
            self.show_grid = !self.show_grid;
        }

        let mouse = ggez::input::mouse::position(ctx);
        let mouse = self.viewport.coordinates_i16(mouse);

        if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
            self.floor_selection = Some(match self.floor_selection {
                Some((start, _)) => (start, mouse),
                None => (mouse, mouse),
            });
        } else {
            self.floor_selection = None; // FIXME set None next frame
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.viewport.set_size(ctx);

        if let Some((start, end)) = self.floor_selection {
            let x = start.x..end.x;
            let y = start.y..end.y;
            self.scene.add_floor(Floor::Floor, North, (x, y));
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer) {
        self.scene.render(tile_renderer);
        tile_renderer.draw(ctx, self.viewport.origin(), self.viewport.scale());

        dbg!(self.floor_selection);

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }
}
