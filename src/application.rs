use iced::{button, executor, keyboard, slider, time,
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
    gui: gui::Gui,
    keyboard: keypad::Keyboard,
    chip8: Option<chip8::Chip8>,
    settings: Chip8EmulatorSettings,
}

pub struct Chip8EmulatorSettings {
    pub rom_name: String,
    pub clock_speed: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    IcedEvent(iced_native::Event),
    Goto(gui::PageModel),
    ChooseRom,
    ClockSpeedChanged(i32),
    BgRedChanged(i32),
    BgGreenChanged(i32),
    BgBlueChanged(i32),
    FgRedChanged(i32),
    FgGreenChanged(i32),
    FgBlueChanged(i32),
    CpuClockTick,
    TimerClockTick,
}

impl Chip8EmulatorSettings {
    pub fn new() -> Self {
        Self {
            rom_name: String::from(""),
            clock_speed: DEFAULT_CLOCK_SPEED,
        }
    }
}

impl Default for Chip8Emulator {
    fn default() -> Self {
        Self {
            gui: gui::Gui::new(),
            keyboard: keypad::Keyboard::new(),
            chip8: None,
            settings: Chip8EmulatorSettings::new(),
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
        if !self.settings.rom_name.is_empty() {
            format!("{} - {}", APPLICATION_TITLE, &self.settings.rom_name)
        } else {
            String::from(APPLICATION_TITLE)
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &mut self.gui.current_page {
            gui::PageModel::MainMenu { .. } => self.gui.make(&self.settings),
            gui::PageModel::EmulationScreen => self.gui.make(&self.settings),
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
                                        self.settings.rom_name = x.to_os_string().into_string().unwrap();
                                        //self.gui.rom_name = Some(x.to_os_string().into_string().unwrap());
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
                            clock_speed_state: slider::State::new(),
                            clock_speed_value: self.settings.clock_speed,
                            bg_red_state: slider::State::new(),
                            bg_red_value: self.gui.screen.bg_red,
                            bg_green_state: slider::State::new(),
                            bg_green_value: self.gui.screen.bg_green,
                            bg_blue_state: slider::State::new(),
                            bg_blue_value: self.gui.screen.bg_blue,
                            fg_red_state: slider::State::new(),
                            fg_red_value: self.gui.screen.bg_red,
                            fg_green_state: slider::State::new(),
                            fg_green_value: self.gui.screen.bg_green,
                            fg_blue_state: slider::State::new(),
                            fg_blue_value: self.gui.screen.bg_blue,
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
                                    self.keyboard.keys[*k] = true;
                                },
                                _ => ()
                            }
                        },
                        keyboard::Event::KeyReleased { key_code, .. } => {
                            match self.keyboard.key_map.get(&key_code) {
                                Some(k) => {
                                    self.keyboard.keys[*k] = false;
                                },
                                _ => ()
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
            Message::ClockSpeedChanged(val) => self.settings.clock_speed = val as u32,
            Message::BgRedChanged(val) => self.gui.screen.bg_red = val as u32,
            Message::BgGreenChanged(val) => self.gui.screen.bg_green = val as u32,
            Message::BgBlueChanged(val) => self.gui.screen.bg_blue = val as u32,
            Message::FgRedChanged(val) => self.gui.screen.fg_red = val as u32,
            Message::FgGreenChanged(val) => self.gui.screen.fg_green = val as u32,
            Message::FgBlueChanged(val) => self.gui.screen.fg_blue = val as u32,
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let runtime_events = iced_native::subscription::events().map(Message::IcedEvent);

        let ticks = time::every(Duration::from_millis(
            1000 / self.settings.clock_speed as u64,
        )).map(|_| -> Message { Message::CpuClockTick });

        let timers = time::every(Duration::from_millis(
            1000 / 60 as u64,
        )).map(|_| -> Message { Message::TimerClockTick });

        Subscription::batch(vec![runtime_events, ticks, timers])
    }
}