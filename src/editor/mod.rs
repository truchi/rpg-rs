use super::*;

mod editor;
mod history;
mod pencil;
mod scene;
mod scene_view;
mod tiles_view;
mod viewport;

pub use editor::*;
pub use history::*;
pub use pencil::*;
pub use scene::*;
pub use scene_view::*;
pub use tiles_view::*;
pub use viewport::*;

macro_rules! elements {
    ($($Name:ident $N:literal [$($Variant:ident $Tile:ident)*])*) => { $(
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub enum $Name { $($Variant,)* }
        pub use $Name::*;

        impl $Name {
            pub fn random() -> Self {
                Self::from_usize(thread_rng().gen_range(0..$N))
            }

            pub fn from_usize(u: usize) -> Self {
                let all = Self::all();

                *unsafe { all.get_unchecked(u % all.len()) }
            }

            pub const fn all() -> [Self; $N] {
                [$(Self::$Variant,)*]
            }

            pub fn tile(&self) -> Tile {
                match self {
                    $(Self::$Variant => Tile::$Tile,)*
                }
            }
        }
    )* };
}

elements!(
    FloorEnum 12 [
        Floor   FLOOR_1
        Cracks1 FLOOR_2
        Cracks2 FLOOR_3
        Cracks3 FLOOR_4
        Cracks4 FLOOR_5
        Cracks5 FLOOR_6
        Cracks6 FLOOR_7
        Cracks7 FLOOR_8
        Ladder  FLOOR_LADDER
        Spikes  FLOOR_SPIKES_ANIM_0
        Hole    HOLE
        Edge    EDGE
    ]
    WallEnum 10 [
        Wall            WALL_MID
        // Column          WALL_COLUMN_MID
        SmallHole       WALL_HOLE_1
        BigHole         WALL_HOLE_2
        RedBanner       WALL_BANNER_RED
        GreenBanner     WALL_BANNER_GREEN
        BlueBanner      WALL_BANNER_BLUE
        YellowBanner    WALL_BANNER_YELLOW
        LavaFountain    WALL_FOUNTAIN_MID_RED_ANIM_0
        WaterFountain   WALL_FOUNTAIN_MID_BLUE_ANIM_0
        Goo             WALL_GOO
    ]
    /*
    Creature 22 [
        // Hero
        MaleElf         ELF_M_IDLE_ANIM_0
        FemaleElf       ELF_F_IDLE_ANIM_0
        MaleKnight      KNIGHT_M_IDLE_ANIM_0
        FemaleKnight    KNIGHT_F_IDLE_ANIM_0
        MaleWizzard     WIZZARD_M_IDLE_ANIM_0
        FemaleWizzard   WIZZARD_F_IDLE_ANIM_0
        MaleLizard      LIZARD_M_IDLE_ANIM_0
        FemaleLizard    LIZARD_F_IDLE_ANIM_0
        // Demons
        Imp             IMP_IDLE_ANIM_0
        Necromancer     NECROMANCER_IDLE_ANIM_0
        Wogol           WOGOL_IDLE_ANIM_0
        Chort           CHORT_IDLE_ANIM_0
        // Undeads
        TinyZombie      TINY_ZOMBIE_IDLE_ANIM_0
        Zombie          ZOMBIE_IDLE_ANIM_0
        IceZombie       ICE_ZOMBIE_IDLE_ANIM_0
        Skelet          SKELET_IDLE_ANIM_0
        // Orcs
        MaskedOrc       MASKED_ORC_IDLE_ANIM_0
        OrcWarrior      ORC_WARRIOR_IDLE_ANIM_0
        OrcShaman       ORC_SHAMAN_IDLE_ANIM_0
        Goblin          GOBLIN_IDLE_ANIM_0
        // Misc
        Muddy           MUDDY_IDLE_ANIM_0
        Swampy          SWAMPY_IDLE_ANIM_0
    ]
    Boss 3 [
        BigDemon        BIG_DEMON_IDLE_ANIM_0  // Demon
        BigZombie       BIG_ZOMBIE_IDLE_ANIM_0 // Undead
        Ogre            OGRE_IDLE_ANIM_0       // Orc
    ]
    */
);

impl Default for WallEnum {
    fn default() -> Self {
        Self::Wall
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Element {
    Floor(FloorEnum),
    Wall(WallEnum),
}

impl Element {
    pub fn tile(&self) -> Tile {
        match self {
            Self::Floor(floor) => floor.tile(),
            Self::Wall(wall) => wall.tile(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}
pub use Orientation::*;

impl Orientation {
    pub fn rotate_right(&mut self) {
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }

    pub fn rotate_left(&mut self) {
        *self = match self {
            North => West,
            East => North,
            South => East,
            West => South,
        };
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Walls {
    pub bottom: Option<WallEnum>,
    pub left:   bool,
    pub right:  bool,
}

impl Walls {
    pub fn new(bottom: Option<WallEnum>, left: bool, right: bool) -> Self {
        Self {
            bottom,
            left,
            right,
        }
    }

    pub fn with_bottom(bottom: Option<WallEnum>) -> Self {
        Self {
            bottom,
            ..Self::default()
        }
    }

    pub fn with_left(left: bool) -> Self {
        Self {
            left,
            ..Self::default()
        }
    }

    pub fn with_right(right: bool) -> Self {
        Self {
            right,
            ..Self::default()
        }
    }

    pub fn tile(grid: [[Self; 3]; 3]) -> [Option<Tile>; 3] {
        let [[tl, t, tr], [ml, m, mr], [bl, b, br]] = grid;

        macro_rules! g {
            ($left:pat, $bottom:pat, $right:pat) => {
                Self { bottom: $bottom, left: $left, right: $right }
            };
            ([$([$(($left:pat, $bottom:pat, $right:pat))*])*]) => {
                [$([$(g!($left, $bottom, $right),)*],)*]
            };
        }

        // if let g!(false, Some(wall), false) = m {
        // return Some(wall.tile());
        // }

        [
            if let Some(wall) = m.bottom {
                Some(wall.tile())
            } else {
                None
            },
            if m.left {
                Some(Tile::WALL_SIDE_MID_RIGHT)
            } else {
                None
            },
            if m.right {
                Some(Tile::WALL_SIDE_MID_LEFT)
            } else {
                None
            },
        ]

        // return match grid {
        // g!([[(_, _, false)(false, None, false)(false, _, _)]
        // [(_, Some(_), false)(false, Some(walls), false)(false, Some(_), _)]
        // [(_, _, false)(false, None, false)(false, _, _)]]) =>
        // Some(walls.tile()), _ => None,
        // };
    }
}
