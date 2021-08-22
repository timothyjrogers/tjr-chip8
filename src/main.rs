use iced::Application;

mod application;
mod gui;
mod keypad;
mod chip8;

fn main() {
    application::Chip8Emulator::run(iced::Settings::default());
}
