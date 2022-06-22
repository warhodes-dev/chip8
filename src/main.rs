use std::{
    error::Error, 
    time::Duration,
    env,
    thread,
};
use clap::{
    Command,
    arg,
};
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

    let options = Command::new("Chip8 Emulator")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Wm.A. Rhodes <warhodes@gmail.com>")
        .about("A simple chip8 emulator")
        .arg(arg!([rom_path] "Path to a chip8 rom (.ch8)"))
        .arg(
            arg!(-l --log_level <LEVEL> "Sets the log level")
            .required(false)
        )
        .get_matches();

    let log_level = match options.value_of("log_level") {
        Some("trace") | Some("5") => log::LevelFilter::Trace,
        Some("debug") | Some("4") => log::LevelFilter::Debug,
        Some("info")  | Some("3") => log::LevelFilter::Info,
        Some("warn")  | Some("2") => log::LevelFilter::Warn,
        _ => log::LevelFilter::Error,
    };

    simple_logger::SimpleLogger::new().with_level(log_level).init()?;

    let rom_path = options.value_of("rom_path").ok_or("usage: chip8 <rom_file>")?;

    let sdl_context = sdl2::init()?;
    let mut video_driver = VideoDriver::new(&sdl_context)?;
    let mut input_driver = InputDriver::new(&sdl_context)?;
    let mut audio_driver = AudioDriver::new(&sdl_context)?;
    let rom = FileDriver::from_string(&rom_path)?;

    let mut cpu = CPU::new();
    cpu.load(&rom.data);

    while input_driver.poll(&mut cpu.kp).is_ok() {
        cpu.step();

        if cpu.fb.update{
            video_driver.draw(&cpu.fb.buf)?;
            cpu.fb.update= false;
        }

        if cpu.sound_state() {
            audio_driver.on();
        } else {
            audio_driver.off();
        }

        thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}


















