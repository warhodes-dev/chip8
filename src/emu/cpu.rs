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
    address_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    kp: Keypad,
    fb: Frame,
}

#[allow(clippy::new_without_default)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0u8; 16],
            memory: [0u8; 4096],
            stack: [0u16; 16],
            address_register: 0u16,
            delay_timer: 0u8,
            sound_timer: 0u8,
            pc: 0u16,
            sp: 0u8,
            kp: Keypad::new(),
            fb: Frame::new(),
        }
    }

    /// Loads a program into memory for execution
    pub fn load(&mut self) {
        unimplemented!();
    }

    /// Increments the state of the emulator by one CPU cycle
    pub fn step(&mut self) {
        unimplemented!();
    }
}
