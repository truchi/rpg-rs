use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Views {
    Scene,
    Tiles,
}

impl Views {
    pub fn switch(&mut self) {
        *self = match self {
            Self::Scene => Self::Tiles,
            Self::Tiles => Self::Scene,
        };
    }
}

#[derive(Debug)]
pub struct Editor {
    keyboard:   Keyboard,
    scene:      Scene,
    tiles_view: TilesView,
    tiles:      Tiles,
    now:        Instant,
    viewport:   Viewport,
    show_grid:  bool,
    view:       Views,
}

impl Editor {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::default();
        scene.make_rects();

        Self {
            keyboard: Keyboard::new(),
            scene,
            tiles_view: TilesView::new(ctx),
            tiles: Tiles::new(ctx),
            now: Instant::now(),
            viewport: Viewport::new(ctx),
            show_grid: true,
            view: Views::Scene,
        }
    }

    fn elapsed(&self) -> Duration {
        self.now.elapsed()
    }

    fn sleep(&self) {
        let rate = Duration::from_secs_f32(RATE);
        let elapsed = self.elapsed();

        if elapsed >= rate {
            let red = "\x1B[0;31m";
            let reset = "\x1B[0m";
            println!(
                "{}Elapsed {:?} >= Rate {:?} !!!{}",
                red, elapsed, rate, reset
            );
        } else {
            std::thread::sleep(rate - elapsed);
        }
    }

    fn handle_switch_view(&mut self) {
        if self.keyboard.is_pressed(KeyCode::Tab) {
            self.view.switch();
        }
    }

    fn update_scene(&mut self, ctx: &mut Context, delta: Duration) {
        self.viewport.handle_keys(&self.keyboard);
        self.handle_keys();

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

    fn update_tiles(&mut self, ctx: &mut Context, delta: Duration) {}

    fn draw_scene(&mut self, ctx: &mut Context) {
        self.scene.render(&mut self.tiles);
        self.tiles.draw(ctx, self.viewport).clear();

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }

    fn draw_tiles(&mut self, ctx: &mut Context) {}

    fn handle_keys(&mut self) {
        let g = self.keyboard.is_pressed(KeyCode::G);

        if g {
            self.show_grid = !self.show_grid;
        }
    }
}

impl EventHandler for Editor {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.viewport.set_size(ctx);
        self.keyboard.update(ctx);

        let now = Instant::now();
        let delta = now - self.now;

        self.handle_switch_view();
        match self.view {
            Views::Scene => {
                self.update_scene(ctx, delta);
            }
            Views::Tiles => {
                self.update_tiles(ctx, delta);
            }
        }

        self.now = now;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(0, 0, 0));

        match self.view {
            Views::Scene => {
                self.draw_scene(ctx);
            }
            Views::Tiles => {
                self.draw_tiles(ctx);
            }
        }

        present(ctx)?;
        self.sleep();
        Ok(())
    }
}
