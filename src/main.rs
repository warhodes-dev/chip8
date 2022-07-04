use std::{
    error::Error, 
    time::{
        Duration, 
        Instant, 
    },
    thread,
};
use chip8::{
    config::Config,
    emu::cpu::CPU, 
    drivers::{
        video::VideoDriver,
        input::InputDriver,
        audio::AudioDriver,
        file::FileDriver,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args()?;

    simple_logger::SimpleLogger::new()
        .with_level(config.log_level)
        .init()?;

    let sdl_context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&sdl_context)?;
    let mut input_driver = InputDriver::new(&sdl_context)?;
    let mut audio_driver = AudioDriver::new(&sdl_context)?;
    let rom = FileDriver::from_string(&config.rom_path)?;

    let mut cpu = CPU::new();
    cpu.load(&rom.data);

    while input_driver.poll(&mut cpu.kp).is_ok() {

        let cycle_start_time = Instant::now();

        for _ in 0..10 {
            cpu.step();
        }
        
        cpu.tick();

        if cpu.fb.update{
            video_driver.draw(&cpu.fb.data)?;
            cpu.fb.update= false;
        }

        if cpu.sound_state() {
            audio_driver.on();
        } else {
            audio_driver.off();
        }

        let cycle_elapsed_time = Instant::now() - cycle_start_time;

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60).saturating_sub(cycle_elapsed_time));
    }

    Ok(())
}
