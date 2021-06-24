use super::*;

#[derive(Copy, Clone, Debug)]
enum Button {
    Click(Point),
    Drag(Rect),
}

impl Button {
    fn new(position: Point) -> Self {
        Self::Click(position)
    }

    fn update(&mut self, position: Point) {
        fn drag(x: f32, y: f32, position: Point) -> Button {
            Button::Drag(Rect {
                x,
                y,
                w: position.x - x,
                h: position.y - y,
            })
        }

        *self = match *self {
            Self::Click(Point { x, y }) => drag(x, y, position),
            Self::Drag(Rect { x, y, .. }) => drag(x, y, position),
        }
    }

    fn click(&self) -> Option<Point> {
        match *self {
            Self::Click(point) => Some(point),
            Self::Drag(_) => None,
        }
    }

    fn drag(&self) -> Option<Rect> {
        match *self {
            Self::Click(_) => None,
            Self::Drag(rect) => Some(rect),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    position: Point,
    left:     Option<Button>,
    right:    Option<Button>,
}

macro_rules! clicks {
    ($($button:ident $click:ident $drag:ident)*) => { $(
        pub fn $button(&self) -> bool {
            self.$button.is_some()
        }

        pub fn $click(&self) -> Option<Point> {
            self.$button.and_then(|button| button.click())
        }

        pub fn $drag(&self) -> Option<Rect> {
            self.$button.and_then(|button| button.drag())
        }
    )* };
}

impl Mouse {
    clicks!(left left_click left_drag right right_click right_drag);

    pub fn new() -> Self {
        Self {
            left:     None,
            right:    None,
            position: [0., 0.].into(),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.position = ggez::input::mouse::position(ctx);

        macro_rules! buttons {
            ($($button:ident ($Button:ident))*) => { $(
                if ggez::input::mouse::button_pressed(ctx, MouseButton::$Button) {
                    if let Some(button) = &mut self.$button {
                        button.update(self.position);
                    } else {
                        self.$button = Some(Button::new(self.position));
                    }
                } else {
                    self.$button = None;
                };
            )* };
        }

        buttons!(left (Left) right (Right));
    }

    pub fn position(&self) -> Point {
        self.position
    }
}
