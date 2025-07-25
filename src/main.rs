use iced::{
    Element, Task,
    widget::{button, column, row},
};
use iced_charts::widget::CandleChart;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(CardExample::new, CardExample::update, CardExample::view)
        .antialiasing(true) //.resizable(false)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
}

#[derive(Debug, Default)]
struct CardExample {}

impl CardExample {
    fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::Submit => {}
        }
    }

    fn view(&self) -> Element<'_, self::Message> {
        let element1: Element<'_, Message> = CandleChart::new().into();
        let element2: Element<'_, Message> = CandleChart::new().into();

        column![
            row![element1, element2],
            button("asd").on_press(Message::Submit)
        ]
        .into()
    }
}
