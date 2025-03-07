use app::Susie;
use linking::Extension;

pub mod app;
pub mod linking;
pub mod structs;

lazy_static::lazy_static! {
    pub static ref EXT: Extension= Extension::load("/home/tpl/projects/susie/test.so").unwrap();
}

fn main() -> iced::Result {
    iced::application("A cool counter", Susie::update, Susie::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}
