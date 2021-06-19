use super::*;

#[derive(Debug)]
pub struct Editor {
    keyboard:  Keyboard,
    scene:     Scene,
    tiles:     Tiles,
    now:       Instant,
    viewport:  Viewport,
    show_grid: bool,
}

impl Editor {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::default();
        scene.make_rects();

        Self {
            keyboard: Keyboard::new(),
            scene,
            tiles: Tiles::new(ctx),
            now: Instant::now(),
            viewport: Viewport::new(ctx),
            show_grid: true,
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

    fn update_delta(&mut self, ctx: &mut Context, delta: Duration) {
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

    fn draw2(&mut self, ctx: &mut Context) {
        self.scene.render(&mut self.tiles);
        self.tiles.draw(ctx, self.viewport).clear();

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }

    pub fn handle_keys(&mut self) {
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
        self.update_delta(ctx, now - self.now);
        self.now = now;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(0, 0, 0));
        self.draw2(ctx);
        present(ctx)?;
        self.sleep();

        Ok(())
    }
}
