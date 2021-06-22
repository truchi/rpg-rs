use super::*;

#[derive(Clone, Default, Debug)]
pub struct History<T> {
    history: Vec<T>, // From newest to oldest
    current: usize,  // < MAX - 1
}

impl<T: Clone> History<T> {
    const MAX: usize = 10;

    pub fn new(t: T) -> Self {
        let mut history = Vec::with_capacity(Self::MAX);
        history.push(t);

        Self {
            history,
            current: 0,
        }
    }

    pub fn get(&self) -> &T {
        debug_assert!(self.history.len() > 0);
        debug_assert!(self.current + 1 < Self::MAX);

        self.history.get(self.current).unwrap()
    }

    pub fn edit(&mut self, f: impl Fn(&mut T)) {
        let mut t = self.get().clone();
        f(&mut t);
        self.add(t);
    }

    pub fn undo(&mut self) -> bool {
        if self.current + 1 < self.history.len() && self.current + 1 < Self::MAX {
            self.current += 1;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    pub fn events(&mut self, keyboard: &Keyboard) {
        let ctrl = keyboard.ctrl();
        let shift = keyboard.shift();
        let z = keyboard.is_pressed(KeyCode::Z);

        if ctrl && z {
            if shift {
                self.redo();
            } else {
                self.undo();
            }
        }
    }

    fn add(&mut self, t: T) {
        // Remove undones
        self.history.splice(0..self.current, []);
        self.current = 0;
        // Insert new
        self.history.insert(0, t);
        // Remove olds
        self.history.truncate(Self::MAX);
    }
}
