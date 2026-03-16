use guitab::tab::Tab;
use iced::Element;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Default)]
struct App {
    tab: Tab,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {}
    }
    fn view(&self) -> Element<'_, Message> {
        iced::widget::space().into()
    }
}

enum Message {}
