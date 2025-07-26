#![windows_subsystem = "windows"]

use iced::{
    Element, Task, Theme,
    widget::{button, column},
};
use iced_charts::{
    candle::{Candle, generate_data},
    widget::CandleChart,
};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(ChartApp::new, ChartApp::update, ChartApp::view)
        .title(ChartApp::title)
        .theme(ChartApp::theme)
        .antialiasing(true)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    AddCandle,
}

#[derive(Debug, Default)]
struct ChartApp {
    candles: Vec<Candle>,
}

impl ChartApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                candles: generate_data(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Trading Charts".into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::AddCandle => {
                if let Some(latest) = self.candles.first() {
                    let open = latest.close;
                    let close = open + (rand::random::<f32>() - 0.5) * 2000.0 * 2.0;
                    let high = open.max(close) + rand::random::<f32>() * 500.0 * 2.0;
                    let low = open.min(close) - rand::random::<f32>() * 500.0 * 2.0;

                    let new_candle = Candle {
                        open,
                        high,
                        low,
                        close,
                    };

                    self.candles.insert(0, new_candle);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, self::Message> {
        column![
            CandleChart::new(self.candles.clone()),
            CandleChart::new(self.candles.clone()),
            button("Add").on_press(Message::AddCandle)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
