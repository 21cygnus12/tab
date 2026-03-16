use guitab::tab::Tab;
use iced::{Element, Length, widget::canvas};

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
        canvas(&self.tab)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

enum Message {}
