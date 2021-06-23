use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ButtonSelection<T = Point> {
    Start(T),
    Select((T, T)),
}

impl ButtonSelection {
    pub fn start(position: Point) -> Self {
        Self::Start(position)
    }

    pub fn select(&mut self, position: Point) {
        *self = Self::Select((self.get_start(), position));
    }

    pub fn get_start(&self) -> Point {
        match *self {
            Self::Start(start) => start,
            Self::Select((start, _)) => start,
        }
    }

    pub fn is_start(&self) -> bool {
        match self {
            Self::Start(_) => true,
            _ => false,
        }
    }

    pub fn ranges(&self) -> (Range<i16>, Range<i16>) {
        match self.as_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { x: ex, y: ey })) => (
                if sx <= ex { sx..ex + 1 } else { ex..sx + 1 },
                if sy <= ey { sy..ey + 1 } else { ey..sy + 1 },
            ),
        }
    }

    pub fn horizontal(&self) -> (Range<i16>, Range<i16>) {
        match self.as_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { x: ex, .. })) =>
                (if sx <= ex { sx..ex + 1 } else { ex..sx + 1 }, sy..sy + 1),
        }
    }

    pub fn vertical(&self) -> (Range<i16>, Range<i16>) {
        match self.as_i16() {
            ButtonSelection::Start(Point { x, y }) => (x..x + 1, y..y + 1),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { y: ey, .. })) =>
                (sx..sx + 1, if sy <= ey { sy..ey + 1 } else { ey..sy + 1 }),
        }
    }

    fn as_i16(&self) -> ButtonSelection<Point<i16>> {
        fn i(point: Point) -> Point<i16> {
            [point.x.floor() as _, point.y.floor() as _].into()
        }

        match *self {
            Self::Start(start) => ButtonSelection::Start(i(start)),
            Self::Select((start, end)) => ButtonSelection::Select((i(start), i(end))),
        }
    }

    pub fn draw(&self, ctx: &mut Context, viewport: Viewport) {
        let (Range { start: sx, end: ex }, Range { start: sy, end: ey }) = self.ranges();
        let Point { x: ox, y: oy } = viewport.origin();
        let Point { x: tx, y: ty } = viewport.tile();
        let w = (ex - sx) as f32 * tx;
        let h = (ey - sy) as f32 * ty;
        let x = ox + sx as f32 * tx + 1.;
        let y = oy + sy as f32 * ty + 1.;

        MeshBuilder::new()
            .rectangle(
                DrawMode::stroke(1.),
                [0., 0., w, h].into(),
                Color::new(1., 0., 0., 1.),
            )
            .unwrap()
            .build(ctx)
            .unwrap()
            .draw(ctx, DrawParam::new().dest([x, y]))
            .unwrap();
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Selection {
    Left(ButtonSelection),
    Right(ButtonSelection),
    None,
}

impl Selection {
    pub fn events(&mut self, mouse: &Mouse, viewport: Viewport, persist: bool) {
        let position = viewport.coordinates(mouse.position());
        let left = mouse.left_press();
        let right = mouse.right_press();

        match self {
            Self::Left(selection) =>
                if left {
                    selection.select(position);
                } else if !persist || right {
                    self.clear()
                },
            Self::Right(selection) =>
                if right {
                    selection.select(position);
                } else if !persist {
                    self.clear()
                },
            Self::None =>
                if left {
                    *self = Self::Left(ButtonSelection::start(position));
                } else if right {
                    *self = Self::Right(ButtonSelection::start(position));
                },
        }
    }

    pub fn clear(&mut self) {
        *self = Self::None;
    }

    pub fn draw(&self, ctx: &mut Context, viewport: Viewport) {
        match self {
            Self::Left(selection) => selection.draw(ctx, viewport),
            Self::Right(selection) => selection.draw(ctx, viewport),
            Self::None => {}
        }
    }
}
