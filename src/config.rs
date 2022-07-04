use std::error::Error;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "Chip8 Emulator")]
#[clap(author = "Wm. A. Rhodes <warhodes@gmail.com>")]
#[clap(about = "A simple chip8 emulator")]
pub struct Cli {

    #[clap(value_parser)]
    rom_path: String,

    #[clap(default_value_t = String::from("off"), short, long, value_parser)]
    log_level: String,

    #[clap(default_value_t = 1.0, short, long, value_parser)]
    speed: f64,

    #[clap(default_value_t = 8, long, value_parser)]
    scale: u32,
}

pub struct Config {
    pub rom_path: String,
    pub log_level: log::LevelFilter,
    pub step_delay: u64,

    /// Unused
    pub scale_factor: u32,
}

impl Config {
    pub fn from_args() -> Result<Self, Box<dyn Error>> {
        let cli = Cli::parse();
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
