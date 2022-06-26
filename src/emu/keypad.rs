/// The internal emulator keypad state.
pub struct Keypad {
    pub state: [bool; 16],
    pub block: bool,
    pub block_reg: usize,
}

#[allow(clippy::new_without_default)]
impl Keypad {
    pub fn new() -> Self {
        Keypad{ state: [false; 16], block: false, block_reg: 0 }
    }

    /// Reset the keypad state to neutral
    pub fn reset(&mut self) {
        self.state.fill(false);
    }
    
    /// Set an individual key to the corresponding state
    pub fn set(&mut self, idx: usize, state: bool) {
        self.state[idx] = state;
    }

    /// Get the current state of the keypad
    pub fn state(&self) -> &[bool; 16] {
        &self.state
    }

    /// Get the current state of a single key
    pub fn state_of(&self, idx: usize) -> bool {
        self.state[idx]
    }
}
