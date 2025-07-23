use iced_charts::app::CandleChart;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(CandleChart::new, CandleChart::update, CandleChart::view)
        // .resizable(false)
        .run()
}
