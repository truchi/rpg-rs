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

    pub fn draw(&self, tile_renderer: &mut TileRenderer, position: Point, viewport: &Viewport) {
        let magnet = viewport.magnetize(position);
        let position = viewport.coordinates(position);
        let x = position.x - magnet.x;

        match *self {
            Pencil::Floor((floor, orientation)) =>
                tile_renderer.add((floor.tile(), magnet, orientation)),
            Pencil::Wall(wall) => tile_renderer.add((
                x_to_wall(
                    x,
                    (),
                    |_| Tile::WALL_SIDE_MID_RIGHT,
                    |_| wall.tile(),
                    |_| Tile::WALL_SIDE_MID_LEFT,
                ),
                magnet,
            )),
        }
    }
}
