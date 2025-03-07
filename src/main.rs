use app::Susie;
use linking::Extension;

pub mod app;
pub mod linking;
pub mod structs;

fn main() -> iced::Result {
    let ext = Extension::load("/home/tpl/projects/susie/test.so").unwrap();

    iced::application("A cool counter", Susie::update, Susie::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}
