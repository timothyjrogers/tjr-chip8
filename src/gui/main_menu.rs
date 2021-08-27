use iced::{button, slider, Align, Button, Color, Column, Container, Element, Length, Row, Rule, Slider, Text, VerticalAlignment};
use crate::application::Message;
use super::PageModel;

pub fn draw<'a>(rom_name: String,
                clock_speed_slider: &'a mut slider::State,
                clock_speed_value: i32,
                bg_red_slider: &'a mut slider::State,
                bg_red_value: i32,
                bg_green_slider: &'a mut slider::State,
                bg_green_value: i32,
                bg_blue_slider: &'a mut slider::State,
                bg_blue_value: i32,
                fg_red_slider: &'a mut slider::State,
                fg_red_value: i32,
                fg_green_slider: &'a mut slider::State,
                fg_green_value: i32,
                fg_blue_slider: &'a mut slider::State,
                fg_blue_value: i32,
                choose_rom_button: &'a mut button::State,
                launch_button: &'a mut button::State) -> Element<'a , Message> {
    let content = Column::new()
        .align_items(Align::Center)
        .push(
        Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Text::new("ROM").size(30)
            )
            .push(
                Button::new(choose_rom_button, Text::new(String::from("Choose Rom")))
                    .on_press(Message::ChooseRom)
            )
            .push(
                Text::new(rom_name.to_string()).size(20).width(Length::Shrink).height(Length::Units(25)),
            )
        )
        .push(
            Rule::horizontal(20)
        )
        .push(
            Column::new()
                .align_items(Align::Center)
                .spacing(20)
                .push(
                    Text::new("Settings")
                        .size(30)
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .push(
                            Text::new("Clock Speed (Hz)").size(20)
                        )
                        .push(
                                Slider::new(clock_speed_slider, 500..=1000, clock_speed_value, Message::ClockSpeedChanged).width(Length::Units(100))
                        )
                        .push(
                            Text::new(clock_speed_value.to_string()).size(20)
                        )
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .align_items(Align::Center)
                        .push(
                            Text::new("Background Color").color(Color::from_rgba(bg_red_value as f32 / 256.0 , bg_green_value as f32 / 256.0, bg_blue_value as f32 / 256.0, 1.0))
                                .vertical_alignment(VerticalAlignment::Center)
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("R")
                                        )
                                        .push(
                                            Slider::new(bg_red_slider, 0..=255, bg_red_value, Message::BgRedChanged).width(Length::Units(80))
                                        )
                                )
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("G")
                                        )
                                        .push(
                                            Slider::new(bg_green_slider, 0..=255, bg_green_value, Message::BgGreenChanged).width(Length::Units(80))
                                        )
                                )
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("B")
                                        )
                                        .push(
                                            Slider::new(bg_blue_slider, 0..=255, bg_blue_value, Message::BgBlueChanged).width(Length::Units(80))
                                        )
                                )
                        )
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .align_items(Align::Center)
                        .push(
                            Text::new("Foreground Color").color(Color::from_rgba(fg_red_value as f32 / 256.0 , fg_green_value as f32 / 256.0, fg_blue_value as f32 / 256.0, 1.0))
                                .vertical_alignment(VerticalAlignment::Center)
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("R")
                                        )
                                        .push(
                                            Slider::new(fg_red_slider, 0..=255, fg_red_value, Message::FgRedChanged).width(Length::Units(80))
                                        )
                                )
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("G")
                                        )
                                        .push(
                                            Slider::new(fg_green_slider, 0..=255, fg_green_value, Message::FgGreenChanged).width(Length::Units(80))
                                        )
                                )
                                .push(
                                    Row::new()
                                        .spacing(15)
                                        .push(
                                            Text::new("B")
                                        )
                                        .push(
                                            Slider::new(fg_blue_slider, 0..=255, fg_blue_value, Message::FgBlueChanged).width(Length::Units(80))
                                        )
                                )
                        )
                )
        )
        .push(
            Rule::horizontal(20)
        )
        .push(
            if &rom_name == "No ROM chosen" {
                    Button::new(launch_button, Text::new(String::from("Launch ROM")))
                } else {
                    Button::new(launch_button, Text::new(String::from("Launch ROM")))
                        .on_press(Message::Goto(PageModel::EmulationScreen))
            }
        );
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}