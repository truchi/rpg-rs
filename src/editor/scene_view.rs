use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Show {
    floors: bool,
    walls:  bool,
    grid:   bool,
}

impl Show {
    pub fn new() -> Self {
        Self {
            floors: true,
            walls:  true,
            grid:   false,
        }
    }

    pub fn floors(&self) -> bool {
        self.floors
    }

    pub fn walls(&self) -> bool {
        self.walls
    }

    pub fn grid(&self) -> bool {
        self.grid
    }

    pub fn show_floors(&mut self) {
        self.floors = true;
    }

    pub fn show_walls(&mut self) {
        self.walls = true;
    }

    pub fn events(&mut self, keyboard: &Keyboard) {
        if keyboard.is_pressed(KeyCode::F) {
            self.floors = !self.floors;
        }
        if keyboard.is_pressed(KeyCode::W) {
            self.walls = !self.walls;
        }
        if keyboard.is_pressed(KeyCode::G) {
            self.grid = !self.grid;
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneView {
    scene:         History<Scene>,
    buffer:        Option<(ButtonSelection, Scene)>,
    pub viewport:  Viewport,
    show:          Show,
    pub selection: Selection,
    pub pencil:    Option<Pencil>,
}

impl SceneView {
    pub fn new(ctx: &mut Context) -> Self {
        let mut scene = Scene::new();
        scene.make_rects();
        let scene = History::new(scene);

        Self {
            scene,
            buffer: None,
            viewport: Viewport::new(ctx),
            show: Show::new(),
            selection: Selection::None,
            pencil: None,
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.handle_keys(keyboard);
        self.show.events(keyboard);
        self.scene.events(keyboard);

        let persist = if let Some(pencil) = &mut self.pencil {
            pencil.events(keyboard);
            false
        } else {
            true
        };
        self.selection.events(mouse, self.viewport, persist);
    }

    pub fn update(&mut self, ctx: &mut Context, keyboard: &Keyboard, mouse: &Mouse) {
        self.viewport.set_size(ctx);

        if let Some(pencil) = self.pencil {
            match pencil {
                Pencil::Floor((floor, orientation)) => {
                    self.show.show_floors();
                    self.update_floor(floor, orientation);
                }
                Pencil::Wall(wall) => {
                    self.show.show_walls();
                    self.update_walls(wall);
                }
            }
        } else if let Selection::Selected(selection) = self.selection {
            let show = self.show;

            if keyboard.is_pressed(KeyCode::R) {
                self.scene.edit(|scene| {
                    scene.rotate_floor(
                        selection.ranges(),
                        if keyboard.shift() {
                            Orientation::rotate_left
                        } else {
                            Orientation::rotate_right
                        },
                    )
                });
            } else if keyboard.is_pressed(KeyCode::Delete) {
                self.scene
                    .edit(|scene| scene.remove(selection.ranges(), show));
                self.selection.clear();
            } else if keyboard.ctrl() && keyboard.is_pressed(KeyCode::X) {
                self.buffer = Some((
                    selection,
                    self.scene.edit(|scene| scene.cut(selection.ranges(), show)),
                ));
                self.selection.clear();
            } else if keyboard.ctrl() && keyboard.is_pressed(KeyCode::C) {
                self.buffer = Some((selection, self.scene.get().copy(selection.ranges(), show)));
                self.selection.clear();
            }
        }

        // if keyboard.ctrl() && keyboard.is_pressed(KeyCode::V) {
        // if let Some((selection, buffer)) = &self.buffer {
        // let start = selection.get_start();
        // let position = self.viewport.coordinates(mouse.position());
        // self.scene.edit(|scene| {
        // scene.paste(buffer.clone(), [position.x - start.x, position.y -
        // start.y]) });
        // }
        // }
    }

    pub fn update_floor(&mut self, floor: FloorEnum, orientation: Orientation) {
        match self.selection {
            Selection::Selecting(selection) => {
                if !selection.is_start() {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    scene.add_floor(floor, orientation, selection.ranges());
                });
            }
            _ => {}
        }
    }

    pub fn update_walls(&mut self, wall: WallEnum) {
        match self.selection {
            Selection::Selecting(selection) => {
                if !selection.is_start() {
                    self.scene.undo();
                }

                self.scene.edit(|scene| {
                    thirds((
                        selection,
                        scene,
                        |scene: &mut Scene| scene.left_wall(true, selection.vertical()),
                        |scene: &mut Scene| scene.bottom_wall(Some(wall), selection.horizontal()),
                        |scene: &mut Scene| scene.right_wall(true, selection.vertical()),
                    ));
                });
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, mut ctx: &mut Context, tile_renderer: &mut TileRenderer, mouse: &Mouse) {
        self.scene.get().render(tile_renderer, self.show);

        if let Some(pencil) = self.pencil {
            if Selection::None == self.selection {
                pencil.draw(tile_renderer, mouse.position(), &self.viewport);
            }
        }

        tile_renderer.draw(ctx, self.viewport.origin(), self.viewport.scale());

        if self.show.grid() {
            Grid::draw(ctx, self.viewport);
        }

        if let Some(selection) = self.selection.selection() {
            match self.pencil {
                Some(Pencil::Wall(_)) => thirds((
                    selection,
                    ctx,
                    |ctx| selection.draw_vertical(ctx, self.viewport),
                    |ctx| selection.draw_horizontal(ctx, self.viewport),
                    |ctx| selection.draw_vertical(ctx, self.viewport),
                )),
                _ => selection.draw(ctx, self.viewport),
            }
        }
    }
}

pub fn thirds<T: ThirdsArgs>(args: T) -> T::Output {
    T::thirds(args)
}

pub trait ThirdsArgs {
    type Output;

    fn thirds(self) -> Self::Output;
}

impl<T, F1, F2, F3, U> ThirdsArgs for (f32, T, F1, F2, F3)
where
    F1: Fn(T) -> U,
    F2: Fn(T) -> U,
    F3: Fn(T) -> U,
{
    type Output = U;

    fn thirds(self) -> Self::Output {
        let x = self.0;
        if 0. <= x && x < 0.33 {
            self.2(self.1)
        } else if 0.33 <= x && x <= 0.67 {
            self.3(self.1)
        } else {
            self.4(self.1)
        }
    }
}

impl<F1, F2, F3, U> ThirdsArgs for (f32, F1, F2, F3)
where
    F1: Fn() -> U,
    F2: Fn() -> U,
    F3: Fn() -> U,
{
    type Output = U;

    fn thirds(self) -> Self::Output {
        (self.0, (), |()| self.1(), |()| self.2(), |()| self.3()).thirds()
    }
}

impl<T, F1, F2, F3, U> ThirdsArgs for (ButtonSelection, T, F1, F2, F3)
where
    F1: Fn(T) -> U,
    F2: Fn(T) -> U,
    F3: Fn(T) -> U,
{
    type Output = U;

    fn thirds(self) -> Self::Output {
        let x = self.0.start().x % 1.;
        let x = if x < 0. { x + 1. } else { x };

        (x, self.1, self.2, self.3, self.4).thirds()
    }
}
