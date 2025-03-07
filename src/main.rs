use app::Susie;
use iced::Theme;
use linking::Extension;

pub mod app;
pub mod linking;
pub mod structs;
pub mod window;

lazy_static::lazy_static! {
    pub static ref EXT: Extension= Extension::load("/home/tpl/projects/susie/test.so").unwrap();
}

fn main() -> iced::Result {
    iced::daemon("hihi", Susie::update, Susie::view)
        .subscription(Susie::subscription)
        .theme(|_, _| Theme::Dark)
        .run_with(Susie::new)
}
