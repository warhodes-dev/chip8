use std::{
    error::Error, 
    time::{Duration, Instant},
    thread,
};
use clap::Parser;
use chip8::{
    emu::cpu::CPU, 
    drivers::{
        video::VideoDriver,
        input::InputDriver,
        audio::AudioDriver,
        file::FileDriver,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(Cli::parse())?;

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
                video_driver.draw(&cpu.fb.buf)?;
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

/* Configure Emulator */

#[derive(Parser)]
#[clap(name = "Chip8 Emulator")]
#[clap(author = "Wm. A. Rhodes <warhodes@gmail.com>")]
#[clap(about = "A simple chip8 emulator")]
struct Cli {
    #[clap(value_parser)]
    rom_path: String,

    #[clap(default_value_t = String::from("off"), short, long, value_parser)]
    log_level: String,

    #[clap(default_value_t = 1.0, short, long, value_parser)]
    speed: f64,

    #[clap(default_value_t = 8, long, value_parser)]
    scale: u32,
}

struct Config {
    rom_path: String,
    log_level: log::LevelFilter,
    step_delay: u64,
    scale_factor: u32,
}

impl Config {
    fn new(cli: Cli) -> Result<Self, Box<dyn Error>> {
        let rom_path = cli.rom_path;

        let log_level = match cli.log_level.as_str() {
            "trace" | "5" => log::LevelFilter::Trace,
            "debug" | "4" => log::LevelFilter::Debug,
            "info"  | "3" => log::LevelFilter::Info,
            "warn"  | "2" => log::LevelFilter::Warn,
            "error" | "1" => log::LevelFilter::Error,
            _ => log::LevelFilter::Off,
        };

        let clock_multi = cli.speed;

        let step_delay = (2.0 / clock_multi) as u64;

        let scale_factor = cli.scale;

        Ok(Config { rom_path, log_level, step_delay, scale_factor })
    }
}
