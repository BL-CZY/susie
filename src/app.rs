use iced::widget::{button, column, row, scrollable, Row};
use iced::{Center, Length, Theme};

#[derive(Default)]
pub struct Susie {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn column_style(theme: &Theme) -> iced::widget::scrollable::Style {
    use iced::widget::scrollable::Style;

    Style {
        container: iced::widget::container::Style {
            text_color: Some(theme.palette().text),
            background: None,
            border: iced::Border {
                color: theme.palette().primary,
                radius: iced::border::Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                },
                width: 1.0,
            },
            shadow: iced::Shadow::default(),
        },
        gap: None,
        horizontal_rail: scrollable::Rail {
            background: None,
            border: iced::Border::default(),
            scroller: scrollable::Scroller {
                color: iced::Color {
                    r: 256.0,
                    g: 256.0,
                    b: 256.0,
                    a: 256.0,
                },
                border: iced::Border {
                    color: iced::Color {
                        r: 256.0,
                        g: 256.0,
                        b: 256.0,
                        a: 256.0,
                    },
                    width: 0.0,
                    radius: iced::border::Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                },
            },
        },
        vertical_rail: scrollable::Rail {
            background: None,
            border: iced::Border::default(),
            scroller: scrollable::Scroller {
                color: theme.palette().primary,
                border: iced::Border {
                    color: theme.palette().primary,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 2.0,
                        top_right: 2.0,
                        bottom_right: 2.0,
                        bottom_left: 2.0,
                    },
                },
            },
        },
    }
}

impl Susie {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn view(&self) -> Row<Message> {
        let left = scrollable(
            column![button("Hi")
                .on_press(Message::Increment)
                .width(Length::Fill)]
            .width(Length::FillPortion(3))
            .padding(10),
        )
        .style(|theme, _| column_style(theme));

        let right = scrollable(
            column![button("Yo")
                .on_press(Message::Decrement)
                .width(Length::Fill)]
            .width(Length::FillPortion(7))
            .padding(10),
        )
        .style(|theme, _| column_style(theme));

        row![left, right]
            .width(Length::Fill)
            .padding(10)
            .align_y(Center)
    }
}
