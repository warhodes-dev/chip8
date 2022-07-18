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

    let rom_path = match config.rom_path {
        Some(rom_path) => {
            log::debug!("rom file provided by cli: {}", rom_path);
            rom_path
        },
        // If no rom is provided by CLI, wait for user to drop a file in
        None => {
            log::debug!("no rom file provided by cli");
            if let Some(path) = input_driver.poll_filedrop() {
                path
            } else {
                return Ok(())
            }
        }
    };
    let rom = FileDriver::from_string(&rom_path)?;

    let mut cpu = CPU::initialize();
    cpu.load(&rom.data);

    while input_driver.poll(&mut cpu).is_ok() {
        let cycle_start_time = Instant::now();

        for _ in 0..10 {
            cpu.step();
        }
        
        cpu.tick();

        if cpu.fb.update{
            video_driver.draw_screen(&cpu.fb.data)?;
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
