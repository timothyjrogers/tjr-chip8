use iced::{button,
           Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Rule, Text};
use crate::application::Message;
use super::PageModel;

pub fn draw<'a>(rom_name: String,
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
                Text::new(rom_name).size(20).width(Length::Shrink).height(Length::Units(25)),
            )
        )
        .push(
            Rule::horizontal(20)
        )
        .push(
            Column::new()
                .align_items(Align::Center)
                .push(
                    Text::new("Settings").size(30)
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .push(
                            Text::new("Clock Speed (Hz)").size(20).horizontal_alignment(HorizontalAlignment::Left)
                        )
                        .push(
                            Text::new("800").size(20).horizontal_alignment(HorizontalAlignment::Right)
                        )
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .push(
                            Text::new("Background Color").horizontal_alignment(HorizontalAlignment::Left)
                        )
                        .push(
                            Text::new("#000000").size(20).horizontal_alignment(HorizontalAlignment::Right)
                        )
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .push(
                            Text::new("Foreground Color").horizontal_alignment(HorizontalAlignment::Left)
                        )
                        .push(
                            Text::new("#ffffff").size(20).horizontal_alignment(HorizontalAlignment::Right)
                        )
                )
        )
        .push(
            Rule::horizontal(20)
        )
        .push(
            Button::new(launch_button, Text::new(String::from("Launch ROM")))
                .on_press(Message::Goto(PageModel::EmulationScreen))
        );
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}