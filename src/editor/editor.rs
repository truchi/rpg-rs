use super::*;

pub trait View {
    fn new(ctx: &mut Context) -> Self;
    fn events(&mut self, keyboard: &Keyboard);
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context, tiles: &mut Tiles);
}

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
    scene_view: SceneView,
    tiles_view: TilesView,
    tiles:      Tiles,
    now:        Instant,
    view:       Views,
}

impl Editor {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            keyboard:   Keyboard::new(),
            scene_view: SceneView::new(ctx),
            tiles_view: TilesView::new(ctx),
            tiles:      Tiles::new(ctx),
            now:        Instant::now(),
            view:       Views::Scene,
        }
    }

    fn handle_switch_view(&mut self) {
        if self.keyboard.is_pressed(KeyCode::Tab) {
            self.view.switch();
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
        self.handle_switch_view();

        match self.view {
            Views::Scene => {
                self.scene_view.events(&self.keyboard);
                self.scene_view.update(ctx);
            }
            Views::Tiles => {
                self.tiles_view.events(&self.keyboard);
                self.tiles_view.update(ctx);
            }
        }

        self.now = now;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(0, 0, 0));

        match self.view {
            Views::Scene => {
                self.scene_view.draw(ctx, &mut self.tiles);
            }
            Views::Tiles => {
                self.tiles_view.draw(ctx, &mut self.tiles);
            }
        }

        present(ctx)?;
        self.sleep();
        Ok(())
    }
}
