use super::*;

#[derive(Clone, Default, Debug)]
pub struct Scene {
    pub floors: HashMap<Point<i16>, (Floor, Orientation)>,
    pub walls:  HashMap<Point<i16>, Walls>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            floors: HashMap::default(),
            walls:  HashMap::default(),
        }
    }

    pub fn render(&self, tile_renderer: &mut TileRenderer) {
        for (pos, &(floor, orientation)) in &self.floors {
            tile_renderer.add((floor.tile(), [pos.x as f32, pos.y as f32], orientation));
        }

        for (pos, walls) in &self.walls {
            if let Some(wall) = walls.top {
                tile_renderer.add((wall.tile(), [pos.x as f32, (pos.y - 1) as f32]));
            }

            if walls.left {
                tile_renderer.add((Tile::WALL_SIDE_MID_RIGHT, [pos.x as f32, pos.y as f32]));
            }

            if walls.right {
                tile_renderer.add((Tile::WALL_SIDE_MID_LEFT, [pos.x as f32, pos.y as f32]));
            }
        }
    }

    pub fn make_rects(&mut self) {
        self.add_floor(Floor::Floor, North, (0..5, 0..5));
        self.walls(Walls::new(None, true, true), (5..10, 5..10));
        self.top_wall(Some(Wall::RedBanner), (10..15, 0..1));
        self.left_wall(true, (10..11, 0..5));
        self.right_wall(true, (15..16, 0..5));
    }

    pub fn add_floor(
        &mut self,
        floor: Floor,
        orientation: Orientation,
        (x, y): (Range<i16>, Range<i16>),
    ) {
        for i in x {
            for j in y.clone() {
                self.floors.insert([i, j].into(), (floor, orientation));
            }
        }
    }

    pub fn remove_floor(&mut self, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                self.floors.remove(&([i, j].into()));
            }
        }
    }

    pub fn rotate_floor(
        &mut self,
        (x, y): (Range<i16>, Range<i16>),
        rotate: impl Fn(&mut Orientation),
    ) {
        for i in x {
            for j in y.clone() {
                self.floors
                    .get_mut(&([i, j].into()))
                    .map(|(_, o)| rotate(o));
            }
        }
    }

    pub fn walls(&mut self, walls: Walls, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                self.walls.insert([i, j].into(), walls);
            }
        }
    }

    pub fn top_wall(&mut self, wall: Option<Wall>, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                if let Some(Walls { top, .. }) = self.walls.get_mut(&([i, j].into())) {
                    *top = wall;
                } else {
                    self.walls.insert([i, j].into(), Walls::with_top(wall));
                }
            }
        }
    }

    pub fn left_wall(&mut self, bool: bool, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                if let Some(Walls { left, .. }) = self.walls.get_mut(&([i, j].into())) {
                    *left = bool;
                } else {
                    self.walls.insert([i, j].into(), Walls::with_left());
                }
            }
        }
    }

    pub fn right_wall(&mut self, bool: bool, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                if let Some(Walls { right, .. }) = self.walls.get_mut(&([i, j].into())) {
                    *right = bool;
                } else {
                    self.walls.insert([i, j].into(), Walls::with_right());
                }
            }
        }
    }
}
