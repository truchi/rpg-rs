use super::*;

#[derive(Clone, Default, Debug)]
pub struct Scene {
    pub floors: HashMap<Point<i16>, Floor>,
}

impl Scene {
    pub fn render(&self, tile_renderer: &mut TileRenderer) {
        for (pos, floor) in &self.floors {
            tile_renderer.add(floor.tile(), [pos.x as f32, pos.y as f32]);
        }
    }

    pub fn make_rects(&mut self) {
        self.make_horizontal(-10, -10, 9);
        self.make_horizontal(-6, -10, 9);
        self.make_horizontal(-5, -10, 9);
        self.make_horizontal(-1, -10, 9);
        self.make_horizontal(0, -10, 9);
        self.make_horizontal(4, -10, 9);
        self.make_horizontal(5, -10, 9);
        self.make_horizontal(9, -10, 9);

        self.make_vertical(-10, -10, 9);
        self.make_vertical(-6, -10, 9);
        self.make_vertical(-5, -10, 9);
        self.make_vertical(-1, -10, 9);
        self.make_vertical(0, -10, 9);
        self.make_vertical(4, -10, 9);
        self.make_vertical(5, -10, 9);
        self.make_vertical(9, -10, 9);

        self.floors.insert([2, 2].into(), Floor::Floor2);
        self.floors.insert([2, 7].into(), Floor::Floor3);
        self.floors.insert([7, 2].into(), Floor::Floor4);
        self.floors.insert([7, 7].into(), Floor::Floor5);
    }

    pub fn make_vertical(&mut self, x: i16, top: i16, bottom: i16) {
        for i in top..=bottom {
            self.floors.insert([x, i].into(), Floor::Floor1);
        }
    }

    pub fn make_horizontal(&mut self, y: i16, left: i16, right: i16) {
        for i in left..=right {
            self.floors.insert([i, y].into(), Floor::Floor1);
        }
    }
}
