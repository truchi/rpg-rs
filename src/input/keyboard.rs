use super::*;

#[derive(Clone, Debug)]
pub struct Keyboard {
    keys: HashMap<KeyCode, (bool, Instant)>,
    mods: KeyMods,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: Default::default(),
            mods: KeyMods::empty(),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let now = Instant::now();
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        self.mods = ggez::input::keyboard::active_mods(ctx);

        // No keys are fresh now, remove olds
        self.keys.retain(|_, (bool, instant)| {
            *bool = false;
            (now - *instant) < KEYBOARD_DEBOUNCE
        });

        // Add missing keys
        for key in keys {
            if self.keys.get(key).is_none() {
                self.keys.insert(*key, (true, now));
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match self.keys.get(&key) {
            Some((true, _)) => true,
            _ => false,
        }
    }

    pub fn is_active(&self, mods: KeyMods) -> bool {
        self.mods.contains(mods)
    }
}
