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
    keyboard:      Keyboard,
    mouse:         Mouse,
    scene_view:    SceneView,
    tiles_view:    TilesView,
    tile_renderer: TileRenderer,
    now:           Instant,
    view:          Views,
    background:    Color,
}

impl Editor {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            keyboard:      Keyboard::new(),
            mouse:         Mouse::new(),
            scene_view:    SceneView::new(ctx),
            tiles_view:    TilesView::new(ctx),
            tile_renderer: TileRenderer::new(ctx),
            now:           Instant::now(),
            view:          Views::Scene,
            background:    Color::BLACK,
        }
    }

    fn events(&mut self) {
        if self.keyboard.is_pressed(KeyCode::Tab) {
            self.view.switch();
        }

        if self.keyboard.is_pressed(KeyCode::B) {
            if self.background == Color::BLACK {
                let mut rng = thread_rng();
                self.background = Color::from_rgb(rng.gen(), rng.gen(), rng.gen());
            } else {
                self.background = Color::BLACK;
            }
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
}

impl EventHandler for Editor {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let now = Instant::now();
        // let delta = now - self.now;

        self.keyboard.update(ctx);
        self.mouse.update(ctx);
        self.events();

        match self.view {
            Views::Scene => {
                self.scene_view.events(ctx, &self.keyboard, &self.mouse);
                self.scene_view.update(ctx);
            }
            Views::Tiles => {
                self.tiles_view.events(ctx, &self.keyboard, &self.mouse);
                self.tiles_view.update(ctx);
            }
        }

        self.now = now;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, self.background);
        self.tile_renderer.clear();

        match self.view {
            Views::Scene => {
                self.scene_view.draw(ctx, &mut self.tile_renderer);
            }
            Views::Tiles => {
                self.tiles_view.draw(ctx, &mut self.tile_renderer);
            }
        }

        present(ctx)?;
        self.sleep();
        Ok(())
    }
}
