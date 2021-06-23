use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Pencil {
    Floor((FloorEnum, Orientation)),
    Wall(WallEnum),
}

impl Pencil {
    pub fn events(&mut self, keyboard: &Keyboard) {
        let shift = keyboard.shift();

        if keyboard.is_pressed(KeyCode::R) {
            if let Self::Floor((_, orientation)) = self {
                if shift {
                    orientation.rotate_left();
                } else {
                    orientation.rotate_right();
                }
            }
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
                Pencil::Wall(wall) => self.update_walls(keyboard, wall),
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

    pub fn update_walls(&mut self, keyboard: &Keyboard, wall: WallEnum) {
        let ctrl = keyboard.ctrl();

        match self.selection {
            Selection::Left(selection) => {
                let x = selection.get_start().x % 1.;
                let x = if x < 0. { x + 1. } else { x };
                let (undo, ranges) = selection.ranges();
                if undo {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    if 0. <= x && x < 0.33 {
                        scene.left_wall(!ctrl, ranges.clone());
                    } else if 0.33 <= x && x <= 0.67 {
                        scene.bottom_wall(if ctrl { None } else { Some(wall) }, ranges.clone());
                    } else {
                        scene.right_wall(!ctrl, ranges.clone());
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
