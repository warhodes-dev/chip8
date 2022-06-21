use std::{
    error::Error, 
    time::Duration,
    thread,
};
use chip8::{
    emu::{
        keypad::Keypad, 
        frame::Frame,
        cpu::CPU,
    }, 
    drivers::{
        video::VideoDriver,
        input::InputDriver,
        audio::AudioDriver,
        file::FileDriver,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;

    let sdl_context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&sdl_context)?;
    let mut input_driver = InputDriver::new(&sdl_context)?;
    let audio_driver = AudioDriver::new(&sdl_context)?;
    let rom = FileDriver::from_string("./ibm.ch8")?;

    let mut cpu = CPU::new();
    cpu.load(&rom.data);

    while input_driver.poll(&mut cpu.keypad).is_ok() {
        cpu.step();

        if cpu.frame_update {
            video_driver.draw(&cpu.frame)?;
            cpu.frame_update = false;
        }

        thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}


















