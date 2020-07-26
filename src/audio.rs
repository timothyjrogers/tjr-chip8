extern crate sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if self.phase >= 0.0 && self.phase < 0.5 {
                *x = self.volume;
            } else {
                *x = -1 as f32 * self.volume;
            }
        }
    }
}

pub struct Audio {
    device: sdl2::audio::AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(ctx: &sdl2::Sdl) -> Audio {
        let audio_subsystem = ctx.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 444.0 / spec.freq as f32,
                phase: 0.0,
                volume: 2.0,
            }
        }).unwrap();
        let audio = Audio {
            device: device,
        };
        return audio;
    }

    pub fn beep(&mut self, on: bool) {
        if on {
            self.device.resume();
        } else {
            self.device.pause();
        }
    }
}