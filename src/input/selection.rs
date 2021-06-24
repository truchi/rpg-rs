use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ButtonSelection<T = f32> {
    Start(Point<T>),
    Select((Point<T>, Point<T>)),
}

impl<T: IntoI16<Output = i16>> ButtonSelection<T> {
    fn select(&mut self, position: Point<T>) {
        *self = Self::Select((self.start(), position));
    }

    pub fn is_start(&self) -> bool {
        match self {
            Self::Start(_) => true,
            _ => false,
        }
    }

    pub fn start(&self) -> Point<T> {
        match *self {
            Self::Start(start) => start,
            Self::Select((start, _)) => start,
        }
    }

    pub fn ranges(&self) -> (Range<i16>, Range<i16>) {
        match self.into_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { x: ex, y: ey })) => (
                if sx <= ex { sx..ex + 1 } else { ex..sx + 1 },
                if sy <= ey { sy..ey + 1 } else { ey..sy + 1 },
            ),
        }
    }

    pub fn horizontal(&self) -> (Range<i16>, Range<i16>) {
        match self.into_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { x: ex, .. })) =>
                (if sx <= ex { sx..ex + 1 } else { ex..sx + 1 }, sy..sy + 1),
        }
    }

    pub fn vertical(&self) -> (Range<i16>, Range<i16>) {
        match self.into_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { y: ey, .. })) =>
                (sx..sx + 1, if sy <= ey { sy..ey + 1 } else { ey..sy + 1 }),
        }
    }

    pub fn draw(&self, ctx: &mut Context, viewport: Viewport) {
        Self::draw_ranges(ctx, viewport, self.ranges());
    }

    pub fn draw_horizontal(&self, ctx: &mut Context, viewport: Viewport) {
        Self::draw_ranges(ctx, viewport, self.horizontal());
    }

    pub fn draw_vertical(&self, ctx: &mut Context, viewport: Viewport) {
        Self::draw_ranges(ctx, viewport, self.vertical());
    }

    fn draw_ranges(ctx: &mut Context, viewport: Viewport, ranges: (Range<i16>, Range<i16>)) {
        let (Range { start: sx, end: ex }, Range { start: sy, end: ey }) = ranges;
        let Point { x: ox, y: oy } = viewport.origin();
        let Point { x: tx, y: ty } = viewport.tile();
        let w = (ex - sx) as f32 * tx;
        let h = (ey - sy) as f32 * ty;
        let x = ox + sx as f32 * tx + 1.;
        let y = oy + sy as f32 * ty + 1.;

        Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.),
            [x, y, w, h].into(),
            Color::new(0., 0., 1., 1.),
        )
        .unwrap()
        .draw(ctx, DrawParam::new())
        .unwrap();
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Selection {
    Selecting(ButtonSelection),
    Selected(ButtonSelection),
    None,
}

impl Selection {
    pub fn start(position: Point) -> Self {
        Self::Selecting(ButtonSelection::Start(position))
    }

    pub fn events(&mut self, mouse: &Mouse, viewport: Viewport, persist: bool) {
        let position = viewport.coordinates(mouse.position());
        let left = mouse.left();
        let right = mouse.right();

        match self {
            Self::Selecting(selection) =>
                if left {
                    selection.select(position);
                } else if persist {
                    *self = Self::Selected(*selection)
                } else {
                    self.clear()
                },
            Self::Selected(_) =>
                if persist {
                    if right {
                        self.clear()
                    }
                } else {
                    self.clear()
                },
            Self::None =>
                if left {
                    *self = Self::start(position);
                },
        }
    }

    pub fn clear(&mut self) {
        *self = Self::None;
    }

    pub fn selection(&self) -> Option<ButtonSelection> {
        match *self {
            Self::Selecting(selection) => Some(selection),
            Self::Selected(selection) => Some(selection),
            Self::None => None,
        }
    }
}

pub trait IntoI16: Copy {
    type Output;
    fn into_i16(&self) -> Self::Output;
}

impl IntoI16 for f32 {
    type Output = i16;

    fn into_i16(&self) -> Self::Output {
        self.floor() as _
    }
}

impl<T: IntoI16<Output = i16>> IntoI16 for Point<T> {
    type Output = Point<i16>;

    fn into_i16(&self) -> Self::Output {
        Point {
            x: self.x.into_i16(),
            y: self.y.into_i16(),
        }
    }
}

impl<T: IntoI16<Output = i16>> IntoI16 for (Point<T>, Point<T>) {
    type Output = (Point<i16>, Point<i16>);

    fn into_i16(&self) -> Self::Output {
        (self.0.into_i16(), self.1.into_i16())
    }
}

impl<T: IntoI16<Output = i16>> IntoI16 for ButtonSelection<T> {
    type Output = ButtonSelection<i16>;

    fn into_i16(&self) -> Self::Output {
        match self {
            Self::Start(start) => ButtonSelection::Start(start.into_i16()),
            Self::Select((start, end)) =>
                ButtonSelection::Select((start.into_i16(), end.into_i16())),
        }
    }
}
