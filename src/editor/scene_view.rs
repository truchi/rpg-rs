use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Selection {
    Start(Point<i16>),
    Select((Point<i16>, Point<i16>)),
}

impl Selection {
    pub fn start(position: Point<i16>) -> Self {
        Self::Start(position)
    }

    pub fn select(&mut self, position: Point<i16>) {
        *self = match *self {
            Self::Start(point) => Self::Select((point, position)),
            Self::Select((start, _)) => Self::Select((start, position)),
        };
    }

    pub fn ranges(&self) -> (bool, (Range<i16>, Range<i16>)) {
        match *self {
            Self::Start(Point { x, y }) => (false, (x..x + 1, y..y + 1)),
            Self::Select((Point { x: sx, y: sy }, Point { x: ex, y: ey })) => (
                true,
                (
                    if sx <= ex { sx..ex + 1 } else { ex..sx + 1 },
                    if sy <= ey { sy..ey + 1 } else { ey..sy + 1 },
                ),
            ),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FloorSelection {
    Left(Selection),
    Right(Selection),
    None,
}

impl FloorSelection {
    pub fn events(&mut self, mouse: &Mouse, viewport: Viewport) {
        let position = viewport.coordinates_i16(mouse.position());
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
                    *self = Self::Left(Selection::start(position));
                } else if right {
                    *self = Self::Right(Selection::start(position));
                },
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:           History<Scene>,
    viewport:        Viewport,
    show_grid:       bool,
    floor_selection: FloorSelection,
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
            floor_selection: FloorSelection::None,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.handle_keys(keyboard);
        self.scene.events(keyboard);
        self.floor_selection.events(mouse, self.viewport);

        if keyboard.is_pressed(KeyCode::G) {
            self.show_grid = !self.show_grid;
        }
    }

    pub fn update(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        let ctrl = keyboard.ctrl();
        let shift = keyboard.shift();

        self.viewport.set_size(ctx);

        match self.floor_selection {
            FloorSelection::Left(selection) => {
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }
                self.scene.edit(|scene| {
                    if ctrl {
                        scene.remove_floor(ranges.clone());
                    } else {
                        scene.add_floor(Floor::Cracks1, North, ranges.clone());
                    }
                });
            }
            FloorSelection::Right(selection) => {
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

#[derive(Clone, Default, Debug)]
pub struct History<T> {
    history: Vec<T>, // From newest to oldest
    current: usize,  // < MAX - 1
}

impl<T: Clone> History<T> {
    const MAX: usize = 10;

    pub fn new(t: T) -> Self {
        let mut history = Vec::with_capacity(Self::MAX);
        history.push(t);

        Self {
            history,
            current: 0,
        }
    }

    pub fn get(&self) -> &T {
        debug_assert!(self.history.len() > 0);
        debug_assert!(self.current + 1 < Self::MAX);

        self.history.get(self.current).unwrap()
    }

    pub fn edit(&mut self, f: impl Fn(&mut T)) {
        let mut t = self.get().clone();
        f(&mut t);
        self.add(t);
    }

    pub fn undo(&mut self) -> bool {
        if self.current + 1 < self.history.len() && self.current + 1 < Self::MAX {
            self.current += 1;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    pub fn events(&mut self, keyboard: &Keyboard) {
        let ctrl = keyboard.ctrl();
        let shift = keyboard.shift();
        let z = keyboard.is_pressed(KeyCode::Z);

        if ctrl && z {
            if shift {
                self.redo();
            } else {
                self.undo();
            }
        }
    }

    fn add(&mut self, t: T) {
        // Remove undones
        self.history.splice(0..self.current, []);
        self.current = 0;
        // Insert new
        self.history.insert(0, t);
        // Remove olds
        self.history.truncate(Self::MAX);
    }
}
