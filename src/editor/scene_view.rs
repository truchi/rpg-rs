use super::*;

#[derive(Copy, Clone, Debug)]
pub enum ButtonSelection<T = Point> {
    Start(T),
    Select((T, T)),
}

impl ButtonSelection {
    pub fn start(position: Point) -> Self {
        Self::Start(position)
    }

    pub fn select(&mut self, position: Point) {
        *self = match *self {
            Self::Start(point) => Self::Select((point, position)),
            Self::Select((start, _)) => Self::Select((start, position)),
        };
    }

    pub fn ranges(&self) -> (bool, (Range<i16>, Range<i16>)) {
        match self.as_i16() {
            ButtonSelection::Start(Point { x, y }) => (false, (x..x + 1, y..y + 1)),
            ButtonSelection::Select((Point { x: sx, y: sy }, Point { x: ex, y: ey })) => (
                true,
                (
                    if sx <= ex { sx..ex + 1 } else { ex..sx + 1 },
                    if sy <= ey { sy..ey + 1 } else { ey..sy + 1 },
                ),
            ),
        }
    }

    fn as_i16(&self) -> ButtonSelection<Point<i16>> {
        fn i(point: Point) -> Point<i16> {
            [point.x.floor() as _, point.y.floor() as _].into()
        }

        match *self {
            Self::Start(point) => ButtonSelection::Start(i(point)),
            Self::Select((start, end)) => ButtonSelection::Select((i(start), i(end))),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Selection {
    Left(ButtonSelection),
    Right(ButtonSelection),
    None,
}

impl Selection {
    pub fn events(&mut self, mouse: &Mouse, viewport: Viewport) {
        let position = viewport.coordinates(mouse.position());
        let left = mouse.left_press();
        let right = mouse.right_press();

        match self {
            Self::Left(selection) =>
                if left {
                    selection.select(position);
                } else {
                    *self = Self::None;
                },
            Self::Right(selection) =>
                if right {
                    selection.select(position);
                } else {
                    *self = Self::None;
                },
            Self::None =>
                if left {
                    *self = Self::Left(ButtonSelection::start(position));
                } else if right {
                    *self = Self::Right(ButtonSelection::start(position));
                },
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:     History<Scene>,
    viewport:  Viewport,
    show_grid: bool,
    selection: Selection,
}

impl SceneView {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::new();
        scene.make_rects();
        let scene = History::new(scene);

        Self {
            scene,
            viewport: Viewport::new(ctx),
            show_grid: false,
            selection: Selection::None,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.handle_keys(keyboard);
        self.scene.events(keyboard);
        self.selection.events(mouse, self.viewport);

        if keyboard.is_pressed(KeyCode::G) {
            self.show_grid = !self.show_grid;
        }
    }

    pub fn update(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        let ctrl = keyboard.ctrl();
        let shift = keyboard.shift();

        self.viewport.set_size(ctx);

        match self.selection {
            Selection::Left(selection) => {
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }
                self.scene.edit(|scene| {
                    if ctrl {
                        scene.remove_floor(ranges.clone());
                    } else {
                        scene.add_floor(Cracks1, North, ranges.clone());
                    }
                });
            }
            Selection::Right(selection) => {
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }

                if ctrl {
                    self.scene.edit(|scene| scene.remove_floor(ranges.clone()));
                } else {
                    self.scene.edit(|scene| {
                        scene.rotate_floor(
                            ranges.clone(),
                            if shift {
                                Orientation::rotate_left
                            } else {
                                Orientation::rotate_right
                            },
                        )
                    });
                }
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer) {
        self.scene.get().render(tile_renderer);
        tile_renderer.draw(ctx, self.viewport.origin(), self.viewport.scale());

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }
}
