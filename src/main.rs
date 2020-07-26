extern crate sdl2;
use sdl2::event::Event;
use std::time::{Duration,Instant};
use sdl2::keyboard::Scancode;
use std::collections::HashSet;
use std::env;
mod cpu;
mod audio;

const SCANCODE_MAP: [Scancode; 16] = [Scancode::X, Scancode::Num1, Scancode::Num2, Scancode::Num3, Scancode::Q, Scancode::W, Scancode::E, Scancode::A, Scancode::S, Scancode::D, Scancode::Z, Scancode::C, Scancode::Num4, Scancode::R, Scancode::F, Scancode::V];

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut audio_subsystem = audio::Audio::new(&sdl_context);

    let mut cpu = cpu::Cpu::new(&sdl_context);
    cpu.load_font();
    cpu.load_rom(rom);

    let mut tick_counter = 0;
    while cpu.running() {
        if tick_counter == 10 {
            tick_counter = 0;
            cpu.decrement_counters();
        }
        //audio_subsystem.beep(cpu.get_beep());
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    cpu.halt();
                },
                _ => {}
            }
        }
        let pressed_keys: HashSet<Scancode> = event_pump.keyboard_state().pressed_scancodes().collect();
        for number in 0..16 {
            cpu.set_keypad(number, pressed_keys.contains(&SCANCODE_MAP[number]));
        }
        let start = Instant::now();
        cpu.tick();
        if cpu.is_waiting() {
            let mut key_pressed = false;
            while !key_pressed {
                let event = event_pump.wait_event();
                match event {
                    Event::KeyDown { scancode, .. } => {
                        match scancode {
                            None => println!("No key press"),
                            Some(code) => {
                                if SCANCODE_MAP.contains(&code) {
                                    for number in 0..16 {
                                        if SCANCODE_MAP[number] == code {
                                            let key_reg = cpu.get_key_reg();
                                            cpu.set_reg(key_reg, number as u8);
                                            cpu.set_waiting(false);
                                            key_pressed = true;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => ()
                }
            }
        }

        let finish = Instant::now();
        let delta = finish.duration_since(start);
        if delta.subsec_micros() < 1429 {
            let wait = Duration::from_micros(1666 - delta.subsec_micros() as u64);
            std::thread::sleep(wait);
        }
        tick_counter = tick_counter + 1;
    }
}