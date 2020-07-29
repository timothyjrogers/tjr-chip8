extern crate sdl2;
use sdl2::keyboard::Scancode;
use std::collections::HashSet;

pub const SCANCODE_MAP: [Scancode; 16] = [Scancode::X, Scancode::Num1, Scancode::Num2, Scancode::Num3, Scancode::Q, Scancode::W, Scancode::E, Scancode::A, Scancode::S, Scancode::D, Scancode::Z, Scancode::C, Scancode::Num4, Scancode::R, Scancode::F, Scancode::V];

pub struct Keypad {
    pub keys: [bool; 16],
    pub key_pressed: bool,
    pub latest_key: u8,
}

impl Keypad {
    pub fn new() -> Keypad {
        return Keypad {
            keys: [false; 16],
            key_pressed: false,
            latest_key: 0,
        };
    }

    pub fn update_pressed_keys(&mut self, state: sdl2::keyboard::KeyboardState) {
        let pressed_keys: HashSet<Scancode> = state.pressed_scancodes().collect();
        for number in 0..16 {
            let pressed = pressed_keys.contains(&SCANCODE_MAP[number]);
            if !self.keys[number] && pressed {
                self.key_pressed = true;
                self.latest_key = number as u8;
            }
            self.keys[number] = pressed;
        }
    }
}