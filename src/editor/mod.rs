use super::*;

mod editor;

pub use editor::*;

#[derive(Copy, Clone, Debug)]
pub enum Floor {
    Floor1,
    Floor2,
    Floor3,
    Floor4,
    Floor5,
    Floor6,
    Floor7,
    Floor8,
}

impl Floor {
    pub fn random() -> Self {
        Self::from_usize(thread_rng().gen_range(0..8))
    }

    pub fn from_usize(u: usize) -> Self {
        match u % 8 {
            0 => Self::Floor1,
            1 => Self::Floor2,
            2 => Self::Floor3,
            3 => Self::Floor4,
            4 => Self::Floor5,
            5 => Self::Floor6,
            6 => Self::Floor7,
            _ => Self::Floor8,
        }
    }

    pub fn tile(&self) -> Tile {
        match self {
            Self::Floor1 => Tile::FLOOR_1,
            Self::Floor2 => Tile::FLOOR_2,
            Self::Floor3 => Tile::FLOOR_3,
            Self::Floor4 => Tile::FLOOR_4,
            Self::Floor5 => Tile::FLOOR_5,
            Self::Floor6 => Tile::FLOOR_6,
            Self::Floor7 => Tile::FLOOR_7,
            Self::Floor8 => Tile::FLOOR_8,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Copy, Clone, Debug)]
pub enum WallFace {
    Wall,
    Column,
    RedBanner,
    GreenBanner,
    BlueBanner,
    YellowBanner,
    LavaFountain,
    WaterFountain,
}

#[derive(Copy, Clone, Debug)]
pub struct Wall {
    front: WallFace,
    back:  WallFace,
}

#[derive(Clone, Debug)]
pub struct Environment {
    xs: Vec<u8>,
}
