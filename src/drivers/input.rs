use std::error::Error;
use sdl2::{
    event::Event,
    keyboard::Keycode, 
    EventPump,
    Sdl,
};

use crate::emu::cpu::CPU;
use crate::drivers::file::FileDriver;

pub struct InputDriver {
    events: EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let events = sdl_context.event_pump()?;
        log::info!("SDL input handler initialized");
        Ok( InputDriver { events } )
    }

    /// Polls the sdl eventpump for events, 
    /// then sets the passed keypad struct to the proper state.
    pub fn poll(&mut self, cpu: &mut CPU) -> Result<(), Box<dyn Error>> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit{..} => {
                    return Err("User terminated SDL context".into()); 
                },
                Event::DropFile {filename, .. } => {
                    let rom = FileDriver::from_string(&filename)?;
                    cpu.reset();
                    cpu.load(&rom.data);
                },
                _ => {},
            }
        }

        let keyboard_state = self.events.keyboard_state();

        // Convert scancodes into keycodes (drop invalid codes)
        let pressed_keys = keyboard_state.pressed_scancodes()
            .filter_map(Keycode::from_scancode);
        
        // Set keypad to true for only pressed keys
        cpu.kp.reset();
        for key in pressed_keys {
            if let Some(idx) = keycode_to_idx(key) {
                cpu.kp.set(idx, true);
            }
        }

        Ok(())
    }

    /// Polls the sdl eventpump for DropFile events
    /// then returns the dropped file's path
    pub fn poll_filedrop(&mut self) -> Result<String, Box<dyn Error>> {

        loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit{..} => { 
                        return Err("User terminated SDL context".into()); 
                    },
                    Event::DropFile{filename, ..} => {
                        return Ok(filename);
                    },
                    _ => {},
                }
            }
        }
    }
}

/// Match keycode to chip-8 keypad idx.
fn keycode_to_idx(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q    => Some(0x4),
        Keycode::W    => Some(0x5),
        Keycode::E    => Some(0x6),
        Keycode::R    => Some(0xD),
        Keycode::A    => Some(0x7),
        Keycode::S    => Some(0x8),
        Keycode::D    => Some(0x9),
        Keycode::F    => Some(0xE),
        Keycode::Z    => Some(0xA),
        Keycode::X    => Some(0x0),
        Keycode::C    => Some(0xB),
        Keycode::V    => Some(0xF),
        _             => None,
    }
}
