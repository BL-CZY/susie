use app::Susie;
use linking::Extension;

pub mod app;
pub mod linking;

fn main() -> iced::Result {
    let ext = Extension::load("/home/tpl/projects/susie/test.so").unwrap();
    assert_eq!(ext.get_ui().unwrap(), "Hello!".to_string());

    iced::application("A cool counter", Susie::update, Susie::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}
