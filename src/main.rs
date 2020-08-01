extern crate sdl2;
use sdl2::event::Event;
use std::time::{Duration,Instant};
use std::env;
mod cpu;
mod audio;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];
    let clock_speed = &args[2].parse::<i32>().unwrap();
    let clock_speed_micros = ((1 as f32 / *clock_speed as f32) * 1_000_000 as f32) as u32;

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cpu = cpu::Cpu::new(&sdl_context,rom);

    loop {
        match cpu.status {
            cpu::CpuStatus::Running => {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => {
                            cpu.halt();
                        },
                        _ => {}
                    }
                }
                let start = Instant::now();
                let kb_state = event_pump.keyboard_state();
                cpu.tick(kb_state);
                let finish = Instant::now();
                let delta = finish.duration_since(start);
                if delta.subsec_micros() < clock_speed_micros {
                    let wait = Duration::from_micros(1666 - delta.subsec_micros() as u64);
                    std::thread::sleep(wait);
                }
            },
            cpu::CpuStatus::Halted => {
                break;
            },
            cpu::CpuStatus::WaitForKey => ()
        }
    }
}