use std::{
    error::Error, 
    time::Duration,
    thread,
};
use chip8::{
    emu::{
        keypad::Keypad, 
        frame::Frame,
    }, 
    drivers::{
        video::VideoDriver,
        input::InputDriver,
        audio::AudioDriver,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init()?;

    let sdl_context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&sdl_context)?;
    let mut input_driver = InputDriver::new(&sdl_context)?;
    let mut audio_driver = AudioDriver::new(&sdl_context)?;

    let mut keypad = Keypad::new();
    let mut frame = Frame::new();

    while input_driver.set_keypad(&mut keypad).is_ok() {
        for (key_idx, state) in keypad.state().iter().enumerate() {
            frame.buf[key_idx / 4][key_idx % 4] = *state;
            
            if key_idx == 0 {
                if *state { 
                    audio_driver.on(); 
                } else { 
                    audio_driver.off(); 
                }
            }
                
        }

        video_driver.draw(&frame)?;

        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}


















