use super::*;

#[derive(Clone, Debug)]
pub struct TilesView {
    selected: Option<usize>,
}

impl TilesView {
    const SCALE: f32 = 2.;

    pub fn new(ctx: &mut Context) -> Self {
        Self { selected: None }
    }

    pub fn selected(&self) -> Option<Tile> {
        if let Some(selected) = self.selected {
            Some(Self::TILES[selected].1)
        } else {
            None
        }
    }

    pub fn events(&mut self, ctx: &mut Context, keyboard: &Keyboard) {
        let click = ggez::input::mouse::button_pressed(ctx, MouseButton::Left);

        if click {
            let pos = ggez::input::mouse::position(ctx);
            for (i, &(Point { x, y }, tile)) in Self::TILES.iter().enumerate() {
                let x = x * TILE_WIDTH;
                let y = y * TILE_HEIGHT;
                let width = Self::SCALE * tile.w as f32;
                let height = Self::SCALE * tile.h as f32;

                if x <= pos.x && pos.x <= x + width {
                    if y <= pos.y && pos.y <= y + height {
                        self.selected = Some(i);
                        break;
                    }
                }
                self.selected = None;
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {}

    pub fn draw(&mut self, ctx: &mut Context, tile_renderer: &mut TileRenderer) {
        for &(point, tile) in Self::TILES {
            tile_renderer.add((tile, point, Self::SCALE));
        }

        tile_renderer.draw(ctx, [0., 0.], 1.);

        if let Some(selected) = self.selected {
            let (Point { x, y }, tile) = Self::TILES[selected];
            let width = Self::SCALE * tile.w as f32;
            let height = Self::SCALE * tile.h as f32;
            let green = Color::new(0., 1., 0., 1.);

            MeshBuilder::new()
                .rectangle(
                    DrawMode::stroke(2.),
                    [
                        x * TILE_WIDTH - 1.,
                        y * TILE_HEIGHT - 1.,
                        width + 2.,
                        height + 2.,
                    ]
                    .into(),
                    green,
                )
                .unwrap()
                .build(ctx)
                .unwrap()
                .draw(ctx, DrawParam::new().dest([0., 0.]))
                .unwrap();
        }
    }
}

impl TilesView {
    const TILES: &'static [(Point, Tile)] = &[
        (Point { x: 1., y: 1. }, Tile::FLOOR_1),
        (Point { x: 4., y: 1. }, Tile::FLOOR_2),
        (Point { x: 7., y: 1. }, Tile::FLOOR_3),
        (Point { x: 10., y: 1. }, Tile::FLOOR_4),
        (Point { x: 13., y: 1. }, Tile::FLOOR_5),
        (Point { x: 16., y: 1. }, Tile::FLOOR_6),
        (Point { x: 19., y: 1. }, Tile::FLOOR_7),
        (Point { x: 22., y: 1. }, Tile::FLOOR_8),
        (Point { x: 25., y: 1. }, Tile::FLOOR_LADDER),
        (Point { x: 28., y: 1. }, Tile::FLOOR_SPIKES_ANIM_0),
        (Point { x: 31., y: 1. }, Tile::HOLE),
        (Point { x: 34., y: 1. }, Tile::EDGE),
        (Point { x: 1., y: 4. }, Tile::WALL_MID),
        (Point { x: 4., y: 4. }, Tile::WALL_COLUMN_MID),
        (Point { x: 7., y: 4. }, Tile::WALL_HOLE_1),
        (Point { x: 10., y: 4. }, Tile::WALL_HOLE_2),
        (Point { x: 13., y: 4. }, Tile::WALL_BANNER_RED),
        (Point { x: 16., y: 4. }, Tile::WALL_BANNER_GREEN),
        (Point { x: 19., y: 4. }, Tile::WALL_BANNER_BLUE),
        (Point { x: 22., y: 4. }, Tile::WALL_BANNER_YELLOW),
        (Point { x: 25., y: 4. }, Tile::WALL_FOUNTAIN_MID_RED_ANIM_0),
        (Point { x: 28., y: 4. }, Tile::WALL_FOUNTAIN_MID_BLUE_ANIM_0),
        (Point { x: 1., y: 7. }, Tile::ELF_M_IDLE_ANIM_0),
        (Point { x: 4., y: 7. }, Tile::ELF_F_IDLE_ANIM_0),
        (Point { x: 7., y: 7. }, Tile::KNIGHT_M_IDLE_ANIM_0),
        (Point { x: 10., y: 7. }, Tile::KNIGHT_F_IDLE_ANIM_0),
        (Point { x: 13., y: 7. }, Tile::WIZZARD_M_IDLE_ANIM_0),
        (Point { x: 16., y: 7. }, Tile::WIZZARD_F_IDLE_ANIM_0),
        (Point { x: 19., y: 7. }, Tile::LIZARD_M_IDLE_ANIM_0),
        (Point { x: 22., y: 7. }, Tile::LIZARD_F_IDLE_ANIM_0),
        (Point { x: 1., y: 11.5 }, Tile::IMP_IDLE_ANIM_0),
        (Point { x: 4., y: 11.5 }, Tile::NECROMANCER_IDLE_ANIM_0),
        (Point { x: 7., y: 11.5 }, Tile::WOGOL_IDLE_ANIM_0),
        (Point { x: 10., y: 11.5 }, Tile::CHORT_IDLE_ANIM_0),
        (Point { x: 13., y: 11.5 }, Tile::TINY_ZOMBIE_IDLE_ANIM_0),
        (Point { x: 16., y: 11.5 }, Tile::ZOMBIE_IDLE_ANIM_0),
        (Point { x: 19., y: 11.5 }, Tile::ICE_ZOMBIE_IDLE_ANIM_0),
        (Point { x: 22., y: 11.5 }, Tile::SKELET_IDLE_ANIM_0),
        (Point { x: 25., y: 11.5 }, Tile::MASKED_ORC_IDLE_ANIM_0),
        (Point { x: 28., y: 11.5 }, Tile::ORC_WARRIOR_IDLE_ANIM_0),
        (Point { x: 31., y: 11.5 }, Tile::ORC_SHAMAN_IDLE_ANIM_0),
        (Point { x: 34., y: 11.5 }, Tile::GOBLIN_IDLE_ANIM_0),
        (Point { x: 37., y: 11.5 }, Tile::MUDDY_IDLE_ANIM_0),
        (Point { x: 40., y: 11.5 }, Tile::SWAMPY_IDLE_ANIM_0),
        (Point { x: 1., y: 15.5 }, Tile::BIG_DEMON_IDLE_ANIM_0),
        (Point { x: 7., y: 15.5 }, Tile::BIG_ZOMBIE_IDLE_ANIM_0),
        (Point { x: 13., y: 15.5 }, Tile::OGRE_IDLE_ANIM_0),
        (Point { x: 1., y: 21. }, Tile::WEAPON_KNIFE),
        (Point { x: 4., y: 21. }, Tile::WEAPON_RUSTY_SWORD),
        (Point { x: 7., y: 21. }, Tile::WEAPON_REGULAR_SWORD),
        (Point { x: 10., y: 21. }, Tile::WEAPON_RED_GEM_SWORD),
        (Point { x: 13., y: 21. }, Tile::WEAPON_BIG_HAMMER),
        (Point { x: 16., y: 21. }, Tile::WEAPON_HAMMER),
        (Point { x: 19., y: 21. }, Tile::WEAPON_BATON_WITH_SPIKES),
        (Point { x: 22., y: 21. }, Tile::WEAPON_MACE),
        (Point { x: 25., y: 21. }, Tile::WEAPON_KATANA),
        (Point { x: 28., y: 21. }, Tile::WEAPON_SAW_SWORD),
        (Point { x: 31., y: 21. }, Tile::WEAPON_ANIME_SWORD),
        (Point { x: 34., y: 21. }, Tile::WEAPON_AXE),
        (Point { x: 37., y: 21. }, Tile::WEAPON_MACHETE),
        (Point { x: 40., y: 21. }, Tile::WEAPON_CLEAVER),
        (Point { x: 43., y: 21. }, Tile::WEAPON_DUEL_SWORD),
        (Point { x: 46., y: 21. }, Tile::WEAPON_KNIGHT_SWORD),
        (Point { x: 49., y: 21. }, Tile::WEAPON_GOLDEN_SWORD),
        (Point { x: 52., y: 21. }, Tile::WEAPON_LAVISH_SWORD),
        (Point { x: 55., y: 21. }, Tile::WEAPON_RED_MAGIC_STAFF),
        (Point { x: 58., y: 21. }, Tile::WEAPON_GREEN_MAGIC_STAFF),
        (Point { x: 61., y: 21. }, Tile::WEAPON_SPEAR),
        (Point { x: 64., y: 21. }, Tile::WEAPON_ARROW),
        (Point { x: 67., y: 21. }, Tile::WEAPON_BOW),
        (Point { x: 1., y: 27. }, Tile::FLASK_BIG_RED),
        (Point { x: 4., y: 27. }, Tile::FLASK_BIG_BLUE),
        (Point { x: 7., y: 27. }, Tile::FLASK_BIG_GREEN),
        (Point { x: 10., y: 27. }, Tile::FLASK_BIG_YELLOW),
        (Point { x: 13., y: 27. }, Tile::FLASK_RED),
        (Point { x: 16., y: 27. }, Tile::FLASK_BLUE),
        (Point { x: 19., y: 27. }, Tile::FLASK_GREEN),
        (Point { x: 22., y: 27. }, Tile::FLASK_YELLOW),
        (Point { x: 25., y: 27. }, Tile::COIN_ANIM_0),
        (Point { x: 28., y: 27. }, Tile::CHEST_EMPTY_OPEN_ANIM_0),
        (Point { x: 31., y: 27. }, Tile::CHEST_FULL_OPEN_ANIM_0),
        (Point { x: 34., y: 27. }, Tile::CHEST_MIMIC_OPEN_ANIM_0),
        (Point { x: 37., y: 27. }, Tile::CRATE),
        (Point { x: 40., y: 27. }, Tile::SKULL),
    ];
}
