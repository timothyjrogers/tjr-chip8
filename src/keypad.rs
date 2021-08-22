use std::collections::HashMap;
use iced::keyboard as iced_keyboard;

pub struct Keyboard {
    pub keys: [bool; 16],
    pub key_map: HashMap<iced_keyboard::KeyCode, usize>
}

impl Keyboard {
    pub fn new() -> Self {
        let key_map: HashMap<iced_keyboard::KeyCode, usize> = [
            (iced_keyboard::KeyCode::Key1, 0x1),
            (iced_keyboard::KeyCode::Key2, 0x2),
            (iced_keyboard::KeyCode::Key3, 0x3),
            (iced_keyboard::KeyCode::Key4, 0xC),
            (iced_keyboard::KeyCode::Q, 0x4),
            (iced_keyboard::KeyCode::W, 0x5),
            (iced_keyboard::KeyCode::E, 0x6),
            (iced_keyboard::KeyCode::R, 0xD),
            (iced_keyboard::KeyCode::A, 0x7),
            (iced_keyboard::KeyCode::S, 0x8),
            (iced_keyboard::KeyCode::D, 0x9),
            (iced_keyboard::KeyCode::F, 0xE),
            (iced_keyboard::KeyCode::Z, 0xA),
            (iced_keyboard::KeyCode::X, 0x0),
            (iced_keyboard::KeyCode::C, 0xB),
            (iced_keyboard::KeyCode::V, 0xF)]
            .iter().cloned().collect();
        Self {
            keys: [false; 16],
            key_map,
        }
    }
}