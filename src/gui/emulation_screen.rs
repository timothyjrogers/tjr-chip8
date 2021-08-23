use iced::{Align, Canvas, Color, Column, Container, Element, Length, Text};
use crate::application::Message;
use super::{Screen};

pub fn draw<'a>(screen: &'a mut Screen, title: String) -> Element<'a , Message> {
    let content = Column::new()
        .align_items(Align::Center)
        .push(
            Text::new(String::from(title))
        )
        .push(
            Canvas::new(screen).width(Length::Units(640)).height(Length::Units(320))
        );
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}