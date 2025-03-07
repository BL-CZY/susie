use iced::{
    widget::{button, column, container, row, scrollable},
    Alignment, Element, Length, Theme,
};

use crate::app::Message;

fn column_style(theme: &Theme) -> iced::widget::container::Style {
    use iced::{border::Radius, widget::container::Style, Border, Color, Shadow, Vector};

    Style {
        border: Border {
            color: theme.palette().primary,
            radius: Radius::new(2.0),
            width: 2.0,
        },
        text_color: None,
        background: Some(iced::Background::Color(theme.palette().background)),
        shadow: Shadow {
            color: Color::BLACK,
            blur_radius: 10.0,
            offset: Vector::new(2.0, 2.0),
        },
    }
}

pub struct Window {}

impl Window {
    pub fn view(&self, _id: iced::window::Id) -> Element<Message> {
        let left = container(scrollable(column![button("Hi")
            .width(Length::Fill)
            .on_press(Message::CreateWindow)]))
        .width(Length::FillPortion(3))
        .height(Length::Fill)
        .padding(5)
        .style(|theme| column_style(theme));

        let right = container(scrollable(column![button("Yo")
            .width(Length::Fill)
            .on_press(Message::CreateWindow)]))
        .width(Length::FillPortion(7))
        .height(Length::Fill)
        .padding(5)
        .style(|theme| column_style(theme))
        .style(|theme| column_style(theme));

        row![left, right]
            .width(Length::Fill)
            .spacing(10)
            .padding(10)
            .align_y(Alignment::Center)
            .into()
    }
}
