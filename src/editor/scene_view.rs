use super::*;

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

    pub fn update(&mut self, ctx: &mut Context, keyboard: &Keyboard) {
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
                if !selection.is_start() {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    if ctrl {
                        scene.remove_floor(selection.ranges());
                    } else {
                        scene.add_floor(floor, orientation, selection.ranges());
                    }
                });
            }
            Selection::Right(selection) => {
                if !selection.is_start() {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    if ctrl {
                        scene.remove_floor(selection.ranges());
                    } else {
                        scene.rotate_floor(
                            selection.ranges(),
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
                if !selection.is_start() {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    x_to_wall(
                        x,
                        scene,
                        |scene| scene.left_wall(!ctrl, selection.vertical()),
                        |scene| {
                            scene.bottom_wall(
                                if ctrl { None } else { Some(wall) },
                                selection.horizontal(),
                            )
                        },
                        |scene| scene.right_wall(!ctrl, selection.vertical()),
                    );
                });
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer, mouse: &Mouse) {
        self.scene.get().render(tile_renderer);

        if let Some(pencil) = self.pencil {
            if Selection::None == self.selection {
                pencil.draw(tile_renderer, mouse.position(), &self.viewport);
            }
        }

        tile_renderer.draw(ctx, self.viewport.origin(), self.viewport.scale());

        if self.show_grid {
            Grid::draw(ctx, self.viewport);
        }
    }
}

pub fn x_to_wall<T, U>(
    x: f32,
    data: T,
    mut left: impl FnMut(T) -> U,
    mut bottom: impl FnMut(T) -> U,
    mut right: impl FnMut(T) -> U,
) -> U {
    if 0. <= x && x < 0.33 {
        left(data)
    } else if 0.33 <= x && x <= 0.67 {
        bottom(data)
    } else {
        right(data)
    }
}
