use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Pencil {
    Floor((FloorEnum, Orientation)),
    BottomWall(WallEnum),
    LeftWall,
    RightWall,
}

impl Pencil {
    pub fn events(&mut self, keyboard: &Keyboard) {
        let shift = keyboard.shift();

        if keyboard.is_pressed(KeyCode::R) {
            *self = match *self {
                Self::Floor((floor, mut orientation)) => Self::Floor((floor, {
                    if shift {
                        orientation.rotate_left();
                    } else {
                        orientation.rotate_right();
                    }
                    orientation
                })),
                Self::BottomWall(wall) =>
                    if shift {
                        Self::RightWall
                    } else {
                        Self::LeftWall
                    },
                Self::LeftWall =>
                    if shift {
                        Self::BottomWall(Default::default())
                    } else {
                        Self::RightWall
                    },
                Self::RightWall =>
                    if shift {
                        Self::LeftWall
                    } else {
                        Self::BottomWall(Default::default())
                    },
            }
        } else {
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:      History<Scene>,
    viewport:   Viewport,
    show_grid:  bool,
    selection:  Selection,
    pub pencil: Option<Pencil>,
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
            pencil: None,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.handle_keys(keyboard);
        self.scene.events(keyboard);
        self.selection.events(mouse, self.viewport);
        if let Some(pencil) = &mut self.pencil {
            pencil.events(keyboard);
        }

        if keyboard.is_pressed(KeyCode::G) {
            self.show_grid = !self.show_grid;
        }
    }

    pub fn update(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.set_size(ctx);

        if let Some(pencil) = self.pencil {
            match pencil {
                Pencil::Floor((floor, orientation)) =>
                    self.update_floor(keyboard, floor, orientation),
                Pencil::BottomWall(wall) =>
                    self.update_walls(keyboard, Scene::bottom_wall, Some(wall)),
                Pencil::LeftWall => self.update_walls(keyboard, Scene::left_wall, true),
                Pencil::RightWall => self.update_walls(keyboard, Scene::right_wall, true),
            }
        }
    }

    pub fn update_floor(
        &mut self,
        keyboard: &Keyboard,
        floor: FloorEnum,
        orientation: Orientation,
    ) {
        let ctrl = keyboard.ctrl();
        let shift = keyboard.shift();

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
                        scene.add_floor(floor, orientation, ranges.clone());
                    }
                });
            }
            Selection::Right(selection) => {
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    if ctrl {
                        scene.remove_floor(ranges.clone())
                    } else {
                        scene.rotate_floor(
                            ranges.clone(),
                            if shift {
                                Orientation::rotate_left
                            } else {
                                Orientation::rotate_right
                            },
                        )
                    }
                });
            }
            _ => {}
        }
    }

    pub fn update_walls<F, T>(&mut self, keyboard: &Keyboard, f: F, arg: T)
    where
        F: Fn(&mut Scene, T, (Range<i16>, Range<i16>)),
        T: Copy + Default,
    {
        let ctrl = keyboard.ctrl();

        match self.selection {
            Selection::Left(selection) => {
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    if ctrl {
                        f(scene, Default::default(), ranges.clone());
                    } else {
                        f(scene, arg, ranges.clone());
                    }
                });
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
