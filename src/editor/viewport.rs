use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    zoom:  u8,
    scale: u8,
    rect:  Rect,
}

impl Viewport {
    pub fn new(ctx: &mut Context) -> Self {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        Self {
            zoom:  0,
            scale: 1,
            rect:  [0., 0., w, h].into(),
        }
    }

    pub fn set_size(&mut self, ctx: &mut Context) {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        self.rect.w = w;
        self.rect.h = h;
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn point(&self) -> Point {
        [self.rect.x, self.rect.y].into()
    }

    pub fn size(&self) -> Point {
        [self.rect.w, self.rect.h].into()
    }

    pub fn x(&self) -> f32 {
        self.rect.x
    }

    pub fn y(&self) -> f32 {
        self.rect.y
    }

    pub fn w(&self) -> f32 {
        self.rect.w
    }

    pub fn h(&self) -> f32 {
        self.rect.h
    }

    pub fn origin(&self) -> Point {
        [-self.rect.x, -self.rect.y].into()
    }

    pub fn zoom(&self) -> u8 {
        self.zoom
    }

    pub fn scale(&self) -> f32 {
        self.scale.into()
    }

    pub fn tile(&self) -> Point {
        [self.scale() * TILE_WIDTH, self.scale() * TILE_HEIGHT].into()
    }

    pub fn zoom_reset(&mut self) {
        while self.zoom_out() {}
    }

    pub fn zoom_in(&mut self) -> bool {
        if self.zoom <= MAX_ZOOM - 1 {
            self.zoom += 1;
            self.scale *= 2;

            self.rect.x += self.w() / 2. - self.origin().x;
            self.rect.y += self.h() / 2. - self.origin().y;
            true
        } else {
            false
        }
    }

    pub fn zoom_out(&mut self) -> bool {
        if self.zoom > 0 {
            self.zoom -= 1;
            self.scale /= 2;

            self.rect.x -= (self.w() / 2. - self.origin().x) / 2.;
            self.rect.y -= (self.h() / 2. - self.origin().y) / 2.;
            true
        } else {
            false
        }
    }

    pub fn translate(&mut self, translate: impl Into<Point>) {
        let translate = translate.into();

        self.rect.x += translate.x;
        self.rect.y += translate.y;
    }

    pub fn handle_keys(&mut self, keyboard: &Keyboard) {
        let plus = keyboard.is_pressed(KeyCode::Plus);
        let minus = keyboard.is_pressed(KeyCode::Minus);
        let page_up = keyboard.is_pressed(KeyCode::PageUp);
        let page_down = keyboard.is_pressed(KeyCode::PageDown);
        let up = keyboard.is_pressed(KeyCode::Up);
        let down = keyboard.is_pressed(KeyCode::Down);
        let left = keyboard.is_pressed(KeyCode::Left);
        let right = keyboard.is_pressed(KeyCode::Right);
        let zero = keyboard.is_pressed(KeyCode::Key0);
        let equals = keyboard.is_pressed(KeyCode::Equals);

        if zero {
            self.rect.x = 0.;
            self.rect.y = 0.;
        }

        if equals {
            self.zoom_reset();
        }

        if plus || page_up {
            self.zoom_in();
        } else if minus || page_down {
            self.zoom_out();
        }

        let steps = 10.;
        if up {
            self.rect.y -= self.rect.h / steps;
        } else if down {
            self.rect.y += self.rect.h / steps;
        } else if left {
            self.rect.x -= self.rect.w / steps;
        } else if right {
            self.rect.x += self.rect.w / steps;
        }
    }
}
