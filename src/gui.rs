use iced::{button, slider,
           canvas::{Cache, Cursor, Fill, Geometry, Program},
           Color, Element, Point, Rectangle, Size, Slider};
use crate::application::Message;

mod main_menu;
mod emulation_screen;

pub struct Gui {
    pub current_page: PageModel,
    pub rom_name: Option<String>,
    pub screen: Screen,
}

struct GuiSettings {
    rom_name: String,
    bg_color: Color,
    fg_color: Color,
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
            rom_name: None,
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

    pub fn make(&mut self, mut clock_speed: u32) -> Element<Message> {
        match &mut self.current_page {
            PageModel::MainMenu { clock_speed_state, clock_speed_value, bg_red_state, bg_red_value, bg_green_state, bg_green_value, bg_blue_state, bg_blue_value, fg_red_state, fg_red_value, fg_green_state, fg_green_value, fg_blue_state, fg_blue_value, choose_rom_button, launch_button } => {
                match & self.rom_name {
                    Some(n) => main_menu::draw(n.to_string(), clock_speed_state, clock_speed as i32, bg_red_state, self.screen.bg_red as i32, bg_green_state, self.screen.bg_green as i32, bg_blue_state, self.screen.bg_blue as i32, fg_red_state, self.screen.fg_red as i32, fg_green_state, self.screen.fg_green as i32, fg_blue_state, self.screen.fg_blue as i32, choose_rom_button, launch_button),
                    None => main_menu::draw("No ROM chosen".to_string(), clock_speed_state, clock_speed as i32, bg_red_state, self.screen.bg_red as i32, bg_green_state, self.screen.bg_green as i32, bg_blue_state, self.screen.bg_blue as i32, fg_red_state, self.screen.fg_red as i32, fg_green_state, self.screen.fg_green as i32, fg_blue_state, self.screen.fg_blue as i32, choose_rom_button, launch_button)
                }
            },
            PageModel::EmulationScreen => {
                match &self.rom_name {
                    Some(x) => emulation_screen::draw(&mut self.screen, String::from(x)),
                    None => emulation_screen::draw(&mut self.screen, String::from(""))
                }
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