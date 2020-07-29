extern crate sdl2;
use sdl2::event::Event;
use std::time::{Duration,Instant};
use std::env;
mod cpu;
mod audio;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cpu = cpu::Cpu::new(&sdl_context,rom);

    let mut tick_counter = 0;
    loop {
        match cpu.status {
            cpu::CpuStatus::Running => {
                if tick_counter == 10 {
                    tick_counter = 0;
                    cpu.decrement_counters();
                }
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
                if delta.subsec_micros() < 1429 {
                    let wait = Duration::from_micros(1666 - delta.subsec_micros() as u64);
                    std::thread::sleep(wait);
                }
                tick_counter = tick_counter + 1;
            },
            cpu::CpuStatus::Halted => {
                break;
            },
            cpu::CpuStatus::WaitForKey => ()
        }
    }
}