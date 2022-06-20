use crate::emu::{
    frame::{
        FB_SIZE,
        Frame,
    },
    keypad::Keypad,
};

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    pc: u16,
    sp: u8,
    kp: Keypad,
    fb: Frame,
}

#[allow(clippy::new_without_default)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            i:  0x200,
            pc: 0x200, //Some programs start at 0x600?
            sp: 0,
            kp: Keypad::new(),
            fb: Frame::new(),
        }
    }

    /// Loads a program into memory for execution
    pub fn load(&mut self, rom: [u8; 4096]) {
        self.memory = rom;
    }

    /// Increments the state of the emulator by one CPU cycle
    pub fn step(&mut self) {
        unimplemented!();
    }
}
