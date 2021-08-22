use iced::{button, executor, keyboard, time,
           Application,Clipboard, Command, Element, Subscription};
use rodio::{
    source::{SineWave, Source},
    Sink,
};
use std::time::Duration;
use nfd2::Response;

use crate::gui;
use crate::chip8;
use crate::keypad;

//CONSTANTS
const DEFAULT_CLOCK_SPEED: u32 = 800;
const APPLICATION_TITLE: &str = "CHIP-8";

//ICED STATE
pub struct Chip8Emulator {
    rom_name: Option<String>,
    gui: gui::Gui,
    keyboard: keypad::Keyboard,
    chip8: Option<chip8::Chip8>,
}

struct ApplicationSettings {
    clock_speed: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    IcedEvent(iced_native::Event),
    Goto(gui::PageModel),
    ChooseRom,
    CpuClockTick,
    TimerClockTick,
}

impl Default for Chip8Emulator {
    fn default() -> Self {
        Self {
            rom_name: None,
            gui: gui::Gui::new(),
            keyboard: keypad::Keyboard::new(),
            chip8: None,
        }
    }
}

    impl Application for Chip8Emulator {
        type Executor = executor::Default;
        type Message = Message;
        type Flags = ();

        fn new(_flags: ()) -> (Self, Command<Message>) {
        (Chip8Emulator::default(), Command::none())
    }

    fn title(&self) -> String {
        match &self.rom_name {
            Some(n) => format!("{} - {}", APPLICATION_TITLE, n),
            None => String::from(APPLICATION_TITLE)
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &mut self.gui.current_page {
            gui::PageModel::MainMenu { .. } => self.gui.make(),
            gui::PageModel::EmulationScreen => self.gui.make(),
        }
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ChooseRom => {
                //This should only ever be reached from MainMenu
                match &self.gui.current_page {
                    gui::PageModel::MainMenu { .. } => {
                        match nfd2::open_file_dialog(None, None).expect("Unable to open file dialog") {
                            Response::Okay(file_path) => {
                                let rom_path = file_path.clone().into_os_string().into_string().unwrap();
                                self.chip8 = Some(chip8::Chip8::new(rom_path));
                                match file_path.file_name() {
                                    Some(x) => {
                                        self.rom_name = Some(x.to_os_string().into_string().unwrap());
                                        self.gui.rom_name = Some(x.to_os_string().into_string().unwrap());
                                    },
                                    None => ()
                                };
                            },
                            _ => println!("User canceled")
                        }
                    },
                    _ => println!("Reached ChooseRom from invalid page")
                }
            },
            Message::Goto(p) => {
                match p {
                    gui::PageModel::MainMenu { .. } => {
                        self.gui.current_page = gui::PageModel::MainMenu {
                            choose_rom_button: button::State::new(),
                            launch_button: button::State::new()
                        };
                    },
                    gui::PageModel::EmulationScreen => {
                        self.gui.current_page = gui::PageModel::EmulationScreen;
                    },
                }
            },
            Message::IcedEvent(event) => {
                match event {
                    iced_native::Event::Keyboard(keyboard_event) => match keyboard_event {
                        keyboard::Event::KeyPressed { key_code, .. } => {
                            match self.keyboard.key_map.get(&key_code) {
                                Some(k) => {
                                    println!("{} pressed", k);
                                    self.keyboard.keys[*k] = true;
                                },
                                _ => println!("Key not used by Chip8")
                            }
                        },
                        keyboard::Event::KeyReleased { key_code, .. } => {
                            match self.keyboard.key_map.get(&key_code) {
                                Some(k) => {
                                    println!("{} released", k);
                                    self.keyboard.keys[*k] = false;
                                },
                                _ => println!("Key not used by Chip8")
                            }
                        },
                        _ => ()
                    },
                    _ => ()
                }
            },
            Message::CpuClockTick => {
                match &mut self.chip8 {
                    Some(chip8) => {
                        match chip8.status {
                            chip8::CpuStatus::Running => {
                                chip8.tick(self.keyboard.keys);
                                if chip8.redraw {
                                    self.gui.screen.pixels = chip8.screen;
                                    self.gui.screen.screen.clear();
                                    chip8.redraw = false;
                                }
                                if chip8.sound > 0 {
                                    std::thread::spawn(move || {
                                        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                                        let sink = Sink::try_new(&stream_handle).unwrap();
                                        let source = SineWave::new(800).take_duration(Duration::from_millis(167));
                                        sink.append(source);
                                        sink.play();
                                        sink.sleep_until_end();
                                    });
                                }
                            }
                            _ => ()
                        }
                    },
                    None => ()
                }
            },
            Message::TimerClockTick => {
                match &mut self.chip8 {
                    Some(chip8) => {
                        match chip8.status {
                            chip8::CpuStatus::Running => chip8.decrement_counters(),
                            _ => ()
                        }
                    },
                    None => ()
                }
            },
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let runtime_events = iced_native::subscription::events().map(Message::IcedEvent);

        let ticks = time::every(Duration::from_millis(
            1000 / DEFAULT_CLOCK_SPEED as u64,
        )).map(|_| -> Message { Message::CpuClockTick });

        let timers = time::every(Duration::from_millis(
            1000 / 60 as u64,
        )).map(|_| -> Message { Message::TimerClockTick });

        Subscription::batch(vec![runtime_events, ticks, timers])
    }
}