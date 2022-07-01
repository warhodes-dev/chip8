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
    let mut video_driver = VideoDriver::new(&sdl_context, config.scale_factor)?;
    let mut input_driver = InputDriver::new(&sdl_context)?;
    let mut audio_driver = AudioDriver::new(&sdl_context)?;
    let rom = FileDriver::from_string(&config.rom_path)?;

    let mut cpu = CPU::new();
    cpu.load(&rom.data);

    let mut total_cycles: u128 = 0;
    let start_time = Instant::now();

    while input_driver.poll(&mut cpu.kp).is_ok() {
        cpu.step();

        // Do not update frame/sound/delay timer when blocking for keypress
        if !cpu.kp.block { 
            total_cycles += 1;

            if cpu.fb.update{
                video_driver.draw_screen(&cpu.fb.buf)?;
                cpu.fb.update= false;
            }

            if cpu.sound_state() {
                audio_driver.on();
            } else {
                audio_driver.off();
            }
        }

        thread::sleep(Duration::from_millis(config.step_delay));
    }

    let time_elapsed = Instant::now() - start_time;
    let cycles_per = total_cycles as f64 / time_elapsed.as_millis() as f64;
    let unit = "millis";
    log::info!("finished emulation");
    log::info!("total cycles: {}", total_cycles);
    log::info!("time elapsed: {}.{}", time_elapsed.as_secs(), time_elapsed.subsec_micros());
    log::info!("cyc/{:8}: {:6}", unit, cycles_per);

    Ok(())
}
