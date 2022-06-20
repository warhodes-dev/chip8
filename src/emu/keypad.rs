/// The internal emulator keypad state.
pub struct Keypad([bool; 16]);

#[allow(clippy::new_without_default)]
impl Keypad {
    pub fn new() -> Self {
        Keypad([false; 16])
    }

    /// Reset the keypad state to neutral
    pub fn reset(&mut self) {
        for key in self.0.iter_mut() {
            *key = false;
        }
    }
    
    /// Set an individual key to the corresponding state
    pub fn set(&mut self, idx: usize, state: bool) {
        self.0[idx] = state;
    }

    /// Get the current state of the keypad
    pub fn state(&self) -> &[bool; 16] {
        &self.0
    }
}
