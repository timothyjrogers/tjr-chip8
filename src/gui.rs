use iced::{button, slider,
           canvas::{Cache, Cursor, Fill, Geometry, Program},
           Color, Element, Point, Rectangle, Size};
use crate::application::{Chip8EmulatorSettings, Message};

mod main_menu;
mod emulation_screen;

pub struct Gui {
    pub current_page: PageModel,
    pub screen: Screen,
}

pub struct Screen {
    pub pixels: [bool; 2048],
    pub bg_red: u32,
    pub bg_green: u32,
    pub bg_blue: u32,
    pub fg_red: u32,
    pub fg_green: u32,
    pub fg_blue: u32,
    pub screen: Cache,
}

#[derive(Debug, Clone)]
pub enum PageModel {
    MainMenu {
        clock_speed_state: slider::State,
        clock_speed_value: u32,
        bg_red_state: slider::State,
        bg_red_value: u32,
        bg_green_state: slider::State,
        bg_green_value: u32,
        bg_blue_state: slider::State,
        bg_blue_value: u32,
        fg_red_state: slider::State,
        fg_red_value: u32,
        fg_green_state: slider::State,
        fg_green_value: u32,
        fg_blue_state: slider::State,
        fg_blue_value: u32,
        choose_rom_button: button::State,
        launch_button: button::State,
    },
    EmulationScreen,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            current_page: PageModel::MainMenu {
                clock_speed_state: slider::State::new(),
                clock_speed_value: 800,
                bg_red_state: slider::State::new(),
                bg_red_value: 0,
                bg_green_state: slider::State::new(),
                bg_green_value: 0,
                bg_blue_state: slider::State::new(),
                bg_blue_value: 0,
                fg_red_state: slider::State::new(),
                fg_red_value: 0,
                fg_green_state: slider::State::new(),
                fg_green_value: 0,
                fg_blue_state: slider::State::new(),
                fg_blue_value: 0,
                choose_rom_button: button::State::new(),
                launch_button: button::State::new(),
            },
            screen: Screen {
                pixels: [false; 2048],
                bg_red: 0,
                bg_green: 0,
                bg_blue: 0,
                fg_red: 0,
                fg_green: 0,
                fg_blue: 0,
                screen: Cache::new(),
            }
        }
    }

    pub fn make(&mut self, settings: &Chip8EmulatorSettings) -> Element<Message> {
        match &mut self.current_page {
            PageModel::MainMenu { clock_speed_state, clock_speed_value: _, bg_red_state, bg_red_value: _, bg_green_state, bg_green_value: _, bg_blue_state, bg_blue_value: _, fg_red_state, fg_red_value: _, fg_green_state, fg_green_value: _, fg_blue_state, fg_blue_value: _, choose_rom_button, launch_button } => {
                main_menu::draw(settings.rom_name.to_string() , clock_speed_state, settings.clock_speed as i32, bg_red_state, self.screen.bg_red as i32, bg_green_state, self.screen.bg_green as i32, bg_blue_state, self.screen.bg_blue as i32, fg_red_state, self.screen.fg_red as i32, fg_green_state, self.screen.fg_green as i32, fg_blue_state, self.screen.fg_blue as i32, choose_rom_button, launch_button)
            },
            PageModel::EmulationScreen => {
                emulation_screen::draw(&mut self.screen, settings.rom_name.to_string())
            }
        }
    }
}


impl Program<Message> for Screen {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let bg_color = Color::from_rgba(self.bg_red as f32 / 256.0 , self.bg_green as f32 / 256.0, self.bg_blue as f32 / 256.0, 1.0);
        let fg_color = Color::from_rgba(self.fg_red as f32 / 256.0 , self.fg_green as f32 / 256.0, self.fg_blue as f32 / 256.0, 1.0);
        let geo = self.screen.draw(bounds.size(), |frame| {
            for i in 0..2048 {
                let mut color = bg_color;
                if self.pixels[(i%64) + (64 * (i/64))] {
                    color = fg_color;
                }
                frame.fill_rectangle(
                    Point::new((i % 64) as f32 * 10.0, (i / 64) as f32 * 10.0),
                    Size::new(10.0, 10.0),
                    Fill::from(color)
                );
            }
        });
        vec![geo]
    }
}