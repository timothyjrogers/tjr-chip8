extern crate sdl2;
use sdl2::event::Event;
use std::time::{Duration,Instant};
use std::env;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::*;
mod cpu;
mod audio;
#[cfg(debug_addssertions)]
use console::Term;

fn main() {
//    #[cfg(debug_addssertions)]
//    println!("This is a debug build.");

    //User input for clock speed
    println!("Input clock speed (600 HZ):");
    let mut clock_input = String::new();
    io::stdin().read_line(&mut clock_input).expect("Unable to read clock speed input");
    let mut clock_hz: f32 = 600.0;
    if !clock_input.trim().is_empty() {
        match clock_input.trim().parse::<f32>() {
            Ok(v) => clock_hz = v,
            Err(e) => panic!(e)
        }
    }
    println!("Running with clock speed {} hz", clock_hz);
    let clock_speed_micros = ((1 as f32 / clock_hz) * 1_000_000 as f32) as u32;
    //Initialize SDL subsystem
    let sdl_context;
    match sdl2::init() {
        Ok(ctx) => sdl_context = ctx,
        Err(e) => panic!(e)
    }
    //Initialize SDL Event Pump
    let mut event_pump;
    match sdl_context.event_pump() {
        Ok(pump) => event_pump = pump,
        Err(e) => panic!(e)
    }

    //Allow user rom choice
    let mut rom_dir;
    match std::env::current_exe() {
        Ok(v) => {
            rom_dir = v;
            rom_dir.pop();
            rom_dir.push("roms");
        },
        Err(e) => panic!("Unable to read ROM directory")
    }
    println!("ROMs location ({}):", rom_dir.to_str().unwrap());
    let mut romdir_input = String::new();
    io::stdin().read_line(&mut romdir_input).expect("Unable to read custom ROM directory speed input");
    if !romdir_input.trim().is_empty() {
        rom_dir = PathBuf::from(romdir_input);
    }

    //Show list of roms in rom_dir for user choice
    let rom_dir_str = rom_dir.to_str().unwrap().trim();
    let roms;
    match read_dir(Path::new(rom_dir_str)) {
        Ok(v) => {
            roms = v.map(|res| res.map(|e| e.file_name().into_string().unwrap()))
                .collect::<Result<Vec<_>>>().unwrap();
        },
        Err(e) => {
            println!("{}", e);
            panic!("Unable to enumerate ROM directory");
        }
    }
    println!("Choose a rom from {}", rom_dir.to_str().unwrap());
    for (i, x) in roms.iter().enumerate() {
        println!("{}. {}", i, x);
    }
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Unable to read choice");
    let mut choice_int: usize;
    match choice.trim().parse::<usize>() {
        Ok(v) => choice_int = v,
        Err(e) => panic!(e)
    }
    let rom_path = Path::new(rom_dir_str).join(&roms[choice_int]);

    println!("You chose: {}", &roms[choice_int]);
    println!("Launching: {}", Path::new(rom_dir_str).join(&roms[choice_int]).to_str().unwrap());
    let mut cpu = cpu::Cpu::new(&sdl_context,rom_path.to_str().unwrap());

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
                cpu.tick(event_pump.keyboard_state());
                let finish = Instant::now();
                let delta = finish.duration_since(start);
                if delta.subsec_micros() < clock_speed_micros {
                    let wait = Duration::from_micros(1666 - delta.subsec_micros() as u64);
                    std::thread::sleep(wait);
                }
            },
            cpu::CpuStatus::Halted => {
                break;
            }
        }
    }
}