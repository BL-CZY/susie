use iced::{
    widget::{button, column, container, row, scrollable, text, Column},
    Alignment, Element, Length, Theme,
};
use smart_default::SmartDefault;

use crate::{app::Message, linking::Extension, EXTS};

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

#[derive(SmartDefault)]
pub struct Window {
    #[default(_code = "iced::window::Id::unique()")]
    pub id: iced::window::Id,
    #[default(_code = "(0, 0)")]
    pub cur_ind: (usize, usize), // (the index of the extension, the index of the entry in the
                                 // extension)
}

fn push_extensions(col: &mut Vec<Element<Message>>, ext: &Extension, ext_ind: usize) {
    col.push(text(ext.name.clone()).into());
    ext.ui.iter().enumerate().for_each(|(ind, ext_ui)| {
        col.push(
            button(text(ext_ui.name.clone()))
                .width(Length::Fill)
                .on_press(Message::EntryClicked(ext_ind, ind))
                .into(),
        );
    });
}

impl Window {
    pub fn view(&self) -> Element<Message> {
        let mut left_col_content: Vec<Element<Message>> = vec![];

        for (ind, ext) in EXTS.iter().enumerate() {
            push_extensions(&mut left_col_content, ext, ind);
        }

        let left = container(scrollable(
            Column::from_vec(left_col_content).width(Length::Fill),
        ))
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
        .style(|theme| column_style(theme));

        row![left, right]
            .width(Length::Fill)
            .spacing(10)
            .padding(10)
            .align_y(Alignment::Center)
            .into()
    }
}
