pub struct Keypad([bool; 16]);

#[allow(clippy::new_without_default)]
impl Keypad {
    pub fn new() -> Self {
        Keypad([false; 16])
    }

    pub fn reset(&mut self) {
        for key in self.0.iter_mut() {
            *key = false;
        }
    }
    
    pub fn set(&mut self, idx: usize, state: bool) {
        self.0[idx] = state;
    }

    pub fn state(&self) -> &[bool; 16] {
        &self.0
    }
}
