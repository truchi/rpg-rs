use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Pencil {
    Floor((FloorEnum, Orientation)),
    BottomWall(WallEnum),
    LeftWall,
    RightWall,
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
                Pencil::BottomWall(_wall) => {}
                Pencil::LeftWall => {}
                Pencil::RightWall => {}
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
            Selection::None => {}
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
