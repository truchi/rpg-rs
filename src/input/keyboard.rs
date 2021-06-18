use super::*;

#[derive(Clone, Default, Debug)]
pub struct Keyboard {
    map: HashMap<KeyCode, (bool, Instant)>,
}

impl Keyboard {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, keys: &HashSet<KeyCode>) {
        let now = Instant::now();

        // No keys are fresh now, remove olds
        self.map.retain(|_, (bool, instant)| {
            *bool = false;
            (now - *instant) < KEYBOARD_DEBOUNCE
        });

        // Add missing keys
        for key in keys {
            if self.map.get(key).is_none() {
                self.map.insert(*key, (true, now));
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match self.map.get(&key) {
            Some((true, _)) => true,
            _ => false,
        }
    }
}
