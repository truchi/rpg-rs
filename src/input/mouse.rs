use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Mouse {
    position: Point,
    left:     Option<bool>,
    middle:   Option<bool>,
    right:    Option<bool>,
}

macro_rules! clicks {
    ($($button:ident: $fresh:ident $press:ident)*) => { $(
        pub fn $fresh(&self) -> bool { self.$button.unwrap_or(false) }
        pub fn $press(&self) -> bool { self.$button.is_some() }
    )* };
}

impl Mouse {
    clicks!(
        left  : left   left_press
        middle: middle middle_press
        right : right  right_press
    );

    pub fn new() -> Self {
        Self {
            left:     None,
            middle:   None,
            right:    None,
            position: [0., 0.].into(),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        macro_rules! buttons {
            ($($button:ident ($Button:ident))*) => { $(
                self.$button = if ggez::input::mouse::button_pressed(ctx, MouseButton::$Button) {
                    Some(match self.$button {
                        Some(_) => false,
                        None => true,
                    })
                } else {
                    None
                };
            )* };
        }

        self.position = ggez::input::mouse::position(ctx);
        buttons!(left (Left) middle (Middle) right (Right));
    }

    pub fn position(&self) -> Point {
        self.position
    }
}
