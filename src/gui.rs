use iced::{button,
           canvas::{Cache, Cursor, Fill, Geometry, Program},
           Color, Element, Point, Rectangle, Size};
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
    bg_color: Color ,
    fg_color: Color,
}

pub struct Screen {
    pub pixels: [bool; 2048],
    pub screen: Cache,
}

#[derive(Debug, Clone)]
pub enum PageModel {
    MainMenu {
        choose_rom_button: button::State,
        launch_button: button::State,
    },
    EmulationScreen,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            current_page: PageModel::MainMenu {
                choose_rom_button: button::State::new(),
                launch_button: button::State::new(),
            },
            rom_name: None,
            screen: Screen {
                pixels: [false; 2048],
                screen: Cache::new(),
            }
        }
    }

    pub fn make(&mut self) -> Element<Message> {
        match &mut self.current_page {
            PageModel::MainMenu { choose_rom_button, launch_button } => {
                match &self.rom_name {
                    Some(n) => main_menu::draw(n.to_string(), choose_rom_button,launch_button),
                    None => main_menu::draw("No ROM chosen".to_string(), choose_rom_button,launch_button)
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
        let geo = self.screen.draw(bounds.size(), |frame| {
            for i in 0..2048 {
                let mut color = Color::BLACK;
                if self.pixels[(i%64) + (64 * (i/64))] {
                    color = Color::WHITE;
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