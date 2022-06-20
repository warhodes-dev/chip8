use std::{
    error::Error,
    time::Duration,
};
use sdl2::{
    audio::{
        AudioCallback,
        AudioDevice,
        AudioSpecDesired,
    }, 
    Sdl,
};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        //Generate a square wave
        for x in out.iter_mut() {
            if self.phase <= 0.5 {
                *x = self.volume;
            } else {
                *x = -self.volume;
            }
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct AudioDriver {
    device: AudioDevice<SquareWave>,
    pub state: bool,
}

impl AudioDriver {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let audio_subsystem = sdl_context.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.03,
            }
        })?;

        Ok(AudioDriver{ device, state: false })
    }

    pub fn on(&mut self) {
        self.device.resume();
    }

    pub fn off(&mut self) {
        self.device.pause();
    }
}
