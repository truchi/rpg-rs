use super::*;

pub struct Editor {
    keyboard:  Keyboard,
    tiles:     Tiles,
    now:       Instant,
    origin:    Point, // In pixels, screen coordinates
    zoom:      u8,    // 0..=MAX_ZOOM
    w:         f32,
    h:         f32,
    show_grid: bool,
}

impl Editor {
    pub fn new(ctx: &mut Context) -> Self {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        Self {
            keyboard: Keyboard::new(),
            tiles: Tiles::new(ctx),
            now: Instant::now(),
            origin: [0., 0.].into(),
            zoom: 0,
            w,
            h,
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

    fn update_delta(&mut self, ctx: &mut Context, delta: Duration) -> GameResult<()> {
        self.handle_keys();
        Ok(())
    }

    fn draw2(&mut self, ctx: &mut Context) -> GameResult<()> {
        // self.handle_zoom(keys);
        // self.handle_show_grid(keys);

        // let pos = ggez::input::mouse::position(ctx);
        // let click = ggez::input::mouse::button_pressed(ctx, MouseButton::Left);
        // if click {
        // dbg!(pos);
        // }

        // for x in 0..30 {
        //     for y in 0..20 {
        //         // let floor = Tile::FLOOR_1;
        //         let floor = Floor::from_usize(x * y + x + y);
        //         // let floor = Floor::random();
        //         self.tiles.add(ctx, floor.tile(), [x as f32, y as f32]);
        //     }
        // }

        // self.tiles.draw(ctx, self.scale()).clear();

        if self.show_grid {
            Grid::draw(ctx, self.origin, self.size(), self.zoom);
        }

        Ok(())
    }

    pub fn handle_keys(&mut self) {
        let plus = self.keyboard.is_pressed(KeyCode::Plus);
        let minus = self.keyboard.is_pressed(KeyCode::Minus);
        let page_up = self.keyboard.is_pressed(KeyCode::PageUp);
        let page_down = self.keyboard.is_pressed(KeyCode::PageDown);
        let up = self.keyboard.is_pressed(KeyCode::Up);
        let down = self.keyboard.is_pressed(KeyCode::Down);
        let left = self.keyboard.is_pressed(KeyCode::Left);
        let right = self.keyboard.is_pressed(KeyCode::Right);
        let g = self.keyboard.is_pressed(KeyCode::G);

        let zoom_translate = |val: &mut f32, max: f32, dir: f32| {
            if *val < max / 2. {
                *val -= dir.signum() * max / 4.;
            } else if *val > max / 2. {
                *val += dir.signum() * max / 4.;
            }
        };

        if plus || page_up {
            if self.zoom <= MAX_ZOOM - 1 {
                self.zoom += 1;
                zoom_translate(&mut self.origin.x, self.w, 1.);
                zoom_translate(&mut self.origin.y, self.h, 1.);
            }
        } else if minus || page_down {
            if self.zoom > 0 {
                self.zoom -= 1;
                zoom_translate(&mut self.origin.x, self.w, -1.);
                zoom_translate(&mut self.origin.y, self.h, -1.);
            }
        }

        if g {
            self.show_grid = !self.show_grid;
        }

        if up {
            self.origin.y += self.h / 10.;
        } else if down {
            self.origin.y -= self.h / 10.;
        } else if left {
            self.origin.x += self.w / 10.;
        } else if right {
            self.origin.x -= self.w / 10.;
        }
    }

    pub fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }

    pub fn scale(&self) -> f32 {
        zoom_to_scale(self.zoom)
    }
}

impl EventHandler for Editor {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // dbg!("UPDATE");
        let (w, h) = ggez::graphics::drawable_size(ctx);
        self.w = w;
        self.h = h;

        self.keyboard
            .update(ggez::input::keyboard::pressed_keys(ctx));

        let now = Instant::now();
        self.update_delta(ctx, now - self.now)?;
        self.now = now;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // dbg!("DRAW");
        clear(ctx, Color::from_rgb(0, 0, 0));
        self.draw2(ctx)?;
        present(ctx)?;
        self.sleep();

        Ok(())
    }
}

pub fn zoom_to_scale(zoom: u8) -> f32 {
    debug_assert!((0..=MAX_ZOOM).contains(&zoom));

    2_u8.pow(zoom as u32) as _
}
