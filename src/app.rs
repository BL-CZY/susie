use std::collections::BTreeMap;

use iced::widget::horizontal_space;
use iced::{window, Element, Subscription, Task, Vector};

use crate::window::Window;

#[derive(Default)]
pub struct Susie {
    windows: BTreeMap<window::Id, Window>,
    cur_ind: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CreateWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
}

impl Susie {
    pub fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings::default());

        (
            Self {
                windows: BTreeMap::new(),
                cur_ind: 0,
            },
            open.map(Message::WindowOpened),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CreateWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::get_position(*last_window)
                    .then(|last_position| {
                        let position =
                            last_position.map_or(window::Position::Default, |last_position| {
                                window::Position::Specific(last_position + Vector::new(20.0, 20.0))
                            });

                        let (_id, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened)
            }
            Message::WindowOpened(id) => {
                let window = Window {};

                self.windows.insert(id, window);
                Task::none()
            }

            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.len() == 0 {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        if let Some(window) = self.windows.get(&window_id) {
            window.view(window_id).into()
        } else {
            horizontal_space().into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}
