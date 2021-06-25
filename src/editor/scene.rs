use super::*;

type SceneFloors = HashMap<Point<i16>, (FloorEnum, Orientation)>;
type SceneWalls = HashMap<Point<i16>, Walls>;

#[derive(Clone, Default, Debug)]
pub struct Scene {
    pub floors: SceneFloors,
    pub walls:  SceneWalls,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            floors: HashMap::default(),
            walls:  HashMap::default(),
        }
    }

    pub fn render(&self, tile_renderer: &mut TileRenderer, show: Show) {
        if show.floors() {
            for (pos, &(floor, orientation)) in &self.floors {
                tile_renderer.add((floor.tile(), [pos.x as f32, pos.y as f32], orientation));
            }
        }

        if show.walls() {
            for (pos, walls) in &self.walls {
                let top_row = [
                    self.walls.get(&top_left(*pos)).copied().unwrap_or_default(),
                    self.walls.get(&top(*pos)).copied().unwrap_or_default(),
                    self.walls
                        .get(&top_right(*pos))
                        .copied()
                        .unwrap_or_default(),
                ];
                let middle_row = [
                    self.walls.get(&left(*pos)).copied().unwrap_or_default(),
                    *walls,
                    self.walls.get(&right(*pos)).copied().unwrap_or_default(),
                ];
                let bottom_row = [
                    self.walls
                        .get(&bottom_left(*pos))
                        .copied()
                        .unwrap_or_default(),
                    self.walls.get(&bottom(*pos)).copied().unwrap_or_default(),
                    self.walls
                        .get(&bottom_right(*pos))
                        .copied()
                        .unwrap_or_default(),
                ];

                for tile in Walls::tile([top_row, middle_row, bottom_row]) {
                    if let Some(tile) = tile {
                        tile_renderer.add((tile, [pos.x as f32, pos.y as f32]));
                    }
                }

                // if let Some(wall) = walls.bottom {
                // tile_renderer.add((wall.tile(), [pos.x as f32, pos.y as
                // f32])); }
                //
                // if walls.left {
                // tile_renderer.add((Tile::WALL_SIDE_MID_RIGHT, [pos.x as f32,
                // pos.y as f32])); }
                //
                // if walls.right {
                // tile_renderer.add((Tile::WALL_SIDE_MID_LEFT, [pos.x as f32,
                // pos.y as f32])); }
            }
        }
    }

    pub fn make_rects(&mut self) {
        self.add_floor(Floor, North, (0..5, 0..5));
        self.walls(Walls::new(None, true, true), (5..10, 5..10));
        self.left_wall(true, (10..11, 0..5));
        self.right_wall(true, (15..16, 0..5));
        self.bottom_wall(Some(RedBanner), (10..16, 10..11));
        self.walls(Walls::new(Some(RedBanner), true, true), (10..16, 5..6));
    }

    pub fn add_floor(
        &mut self,
        floor: FloorEnum,
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

    pub fn copy_floors(&self, (x, y): (Range<i16>, Range<i16>)) -> SceneFloors {
        let mut copy = SceneFloors::new();

        for i in x {
            for j in y.clone() {
                if let Some(&floor) = self.floors.get(&[i, j].into()) {
                    copy.insert([i, j].into(), floor);
                }
            }
        }

        copy
    }

    pub fn paste_floors(&mut self, floors: SceneFloors, delta: impl Into<Point<i16>>) {
        let delta = delta.into();

        for (Point { x, y }, floor) in floors {
            self.floors.insert([x + delta.x, y + delta.y].into(), floor);
        }
    }

    pub fn walls(&mut self, walls: Walls, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                self.walls.insert([i, j].into(), walls);
            }
        }
    }

    pub fn bottom_wall(&mut self, wall: Option<WallEnum>, (x, y): (Range<i16>, Range<i16>)) {
        for i in x {
            for j in y.clone() {
                if let Some(Walls { bottom, .. }) = self.walls.get_mut(&([i, j].into())) {
                    *bottom = wall;
                } else {
                    self.walls.insert([i, j].into(), Walls::with_bottom(wall));
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
                    self.walls.insert([i, j].into(), Walls::with_left(bool));
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
                    self.walls.insert([i, j].into(), Walls::with_right(bool));
                }
            }
        }
    }

    pub fn copy_walls(&self, (x, y): (Range<i16>, Range<i16>)) -> SceneWalls {
        let mut copy = SceneWalls::new();

        for i in x {
            for j in y.clone() {
                if let Some(&walls) = self.walls.get(&[i, j].into()) {
                    copy.insert([i, j].into(), walls);
                }
            }
        }

        copy
    }

    pub fn paste_walls(&mut self, walls: SceneWalls, delta: impl Into<Point<i16>>) {
        let delta = delta.into();

        for (Point { x, y }, walls) in walls {
            self.walls.insert([x + delta.x, y + delta.y].into(), walls);
        }
    }

    pub fn remove(&mut self, ranges: (Range<i16>, Range<i16>), show: Show) {
        if show.floors() {
            self.remove_floor(ranges.clone())
        }

        if show.walls() {
            self.bottom_wall(None, ranges.clone());
            self.left_wall(false, ranges.clone());
            self.right_wall(false, ranges.clone());
        }
    }

    pub fn cut(&mut self, ranges: (Range<i16>, Range<i16>), show: Show) -> Self {
        let copy = self.copy(ranges.clone(), show);
        self.remove(ranges, show);
        copy
    }

    pub fn copy(&self, ranges: (Range<i16>, Range<i16>), show: Show) -> Self {
        let mut copy = Self::new();

        if show.floors() {
            copy.floors = self.copy_floors(ranges.clone());
        }

        if show.walls() {
            copy.walls = self.copy_walls(ranges.clone());
        }

        copy
    }

    pub fn paste(&mut self, scene: Self, delta: impl Into<Point<i16>>) {
        let delta = delta.into();

        self.paste_floors(scene.floors, delta);
        self.paste_walls(scene.walls, delta);
    }
}

pub fn top(Point { x, y }: Point<i16>) -> Point<i16> {
    Point { x, y: y - 1 }
}

pub fn bottom(Point { x, y }: Point<i16>) -> Point<i16> {
    Point { x, y: y + 1 }
}

pub fn left(Point { x, y }: Point<i16>) -> Point<i16> {
    Point { x: x - 1, y }
}

pub fn right(Point { x, y }: Point<i16>) -> Point<i16> {
    Point { x: x + 1, y }
}

pub fn top_left(point: Point<i16>) -> Point<i16> {
    top(left(point))
}

pub fn top_right(point: Point<i16>) -> Point<i16> {
    top(right(point))
}

pub fn bottom_left(point: Point<i16>) -> Point<i16> {
    bottom(left(point))
}

pub fn bottom_right(point: Point<i16>) -> Point<i16> {
    bottom(right(point))
}
