use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    scale: f32,
    rect:  Rect,
}

impl Viewport {
    pub fn new(ctx: &mut Context) -> Self {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        Self {
            scale: 1.,
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

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn tile(&self) -> Point {
        [self.scale() * TILE_WIDTH, self.scale() * TILE_HEIGHT].into()
    }

    pub fn zoom(&mut self, point: impl Into<Point>, delta: f32) {
        if self.scale == 0. {
            self.scale = 1.;
            return;
        } else if -delta >= self.scale {
            return;
        }

        let factor = (self.scale + delta) / self.scale;
        let point = point.into();
        let translate = Point {
            x: (factor - 1.) * (point.x - self.origin().x),
            y: (factor - 1.) * (point.y - self.origin().y),
        };

        self.scale *= factor;
        self.translate(translate);
    }

    pub fn zoom_in(&mut self) {
        self.scale *= 2.;

        self.rect.x += self.w() / 2. - self.origin().x;
        self.rect.y += self.h() / 2. - self.origin().y;
    }

    pub fn zoom_out(&mut self) {
        self.scale /= 2.;

        self.rect.x -= (self.w() / 2. - self.origin().x) / 2.;
        self.rect.y -= (self.h() / 2. - self.origin().y) / 2.;
    }

    pub fn translate(&mut self, translate: impl Into<Point>) {
        let translate = translate.into();

        self.rect.x += translate.x;
        self.rect.y += translate.y;
    }

    pub fn handle_keys(&mut self, keyboard: &Keyboard) {
        let ctrl = keyboard.ctrl();
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

        if plus || page_up {
            self.zoom_in();
        } else if minus || page_down {
            self.zoom_out();
        }

        let steps = if ctrl { 1. } else { 10. };
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

    pub fn coordinates(&self, position: impl Into<Point>) -> Point {
        let position = position.into();
        let origin = self.origin();
        let scale = self.scale();

        [
            (position.x - origin.x) / (scale * TILE_WIDTH),
            (position.y - origin.y) / (scale * TILE_HEIGHT),
        ]
        .into()
    }

    pub fn coordinates_i16(&self, position: impl Into<Point>) -> Point<i16> {
        let Point { x, y } = self.coordinates(position);

        [x.floor() as _, y.floor() as _].into()
    }

    pub fn magnetize(&self, position: impl Into<Point>) -> Point {
        let position = position.into();

        Point {
            x: ((position.x - self.origin().x) / self.tile().x).floor(),
            y: ((position.y - self.origin().y) / self.tile().y).floor(),
        }
    }
}
