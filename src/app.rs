use iced::{
    Color, Element, Fill, Point, Rectangle, Renderer, Size, Task, Theme, mouse,
    widget::{
        Action,
        canvas::{Cache, Geometry, LineDash, Path, Stroke, Style},
    },
};

use iced::widget::canvas;

use crate::{
    candles::{Candle, generate_data},
    viewport::{self, ViewportManager},
};

pub struct CandleChart {
    chart_cache: Cache,
    overlay_cache: Cache,
    viewport: ViewportManager,
    data: Vec<Candle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Viewport(viewport::Message),
}

impl CandleChart {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                chart_cache: Cache::default(),
                overlay_cache: Cache::default(),
                viewport: ViewportManager::new(),
                data: generate_data(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Viewport(msg) => {
                if self.viewport.update(msg) {
                    self.chart_cache.clear();
                    self.overlay_cache.clear();
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        canvas(self).width(Fill).height(Fill).into()
    }
}

impl CandleChart {
    const CANDLE_SPACE: f32 = 10.0;
    const CANDLE_WIDTH: f32 = 4.0;

    fn get_candle_color(candle: &Candle) -> Color {
        if candle.close > candle.open {
            Color::from_rgb(0.03, 0.6, 0.5)
        } else {
            Color::from_rgb(0.95, 0.21, 0.27)
        }
    }

    fn draw_candle(
        &self,
        frame: &mut canvas::Frame,
        candle: &Candle,
        index: usize,
        bounds: &Rectangle,
    ) {
        let base_x = index as f32 * Self::CANDLE_SPACE;
        let color = Self::get_candle_color(candle);

        let top = self.viewport.transform(base_x, candle.high, bounds);
        let bottom = self.viewport.transform(base_x, candle.low, bounds);
        let wick = Path::line(top, bottom);

        frame.stroke(&wick, Stroke::default().with_width(2.0).with_color(color));

        let opos = self
            .viewport
            .transform(base_x - Self::CANDLE_WIDTH, candle.open, bounds);
        let cpos = self
            .viewport
            .transform(base_x + Self::CANDLE_WIDTH, candle.close, bounds);

        let rect_pos = Point::new(opos.x.min(cpos.x), opos.y.min(cpos.y));
        let rect_size = Size::new((cpos.x - opos.x).abs(), (cpos.y - opos.y).abs());

        let body = Path::rectangle(rect_pos, rect_size);
        frame.fill(&body, color);
    }

    fn draw_price_line(&self, frame: &mut canvas::Frame, window: &Rectangle, bounds: &Rectangle) {
        if self.data.is_empty() {
            return;
        }

        let candle = &self.data[0];
        let price = candle.close;
        let color = Self::get_candle_color(candle);

        let stroke = Stroke {
            width: 1.0,
            line_dash: LineDash {
                segments: &[1.0, 2.0],
                offset: 0,
            },
            style: Style::Solid(color),
            ..Stroke::default()
        };

        let start = self.viewport.transform(window.x, price, bounds);
        let end = self
            .viewport
            .transform(window.x + window.width, price, bounds);
        let line = Path::line(start, end);

        frame.stroke(&line, stroke);
    }

    fn draw_crosshair(
        &self,
        frame: &mut canvas::Frame,
        cursor_pos: Point,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        let stroke = Stroke {
            width: 1.0,
            line_dash: LineDash {
                segments: &[5.0, 6.0],
                offset: 0,
            },
            style: Style::Solid(Color::from_rgb(0.3, 0.3, 0.3)),
            ..Stroke::default()
        };

        let mut snap_x = window.x + window.width * (cursor_pos.x / bounds.width);
        snap_x = (snap_x / Self::CANDLE_SPACE).round() * Self::CANDLE_SPACE;

        let v_start = self.viewport.transform(snap_x, window.y, bounds);
        let v_end = self
            .viewport
            .transform(snap_x, window.y + window.height, bounds);
        let v_line = Path::line(v_start, v_end);
        frame.stroke(&v_line, stroke);

        let price = window.y + window.height * (1.0 - cursor_pos.y / bounds.height);
        let h_start = self.viewport.transform(window.x, price, bounds);
        let h_end = self
            .viewport
            .transform(window.x + window.width, price, bounds);
        let h_line = Path::line(h_start, h_end);
        frame.stroke(&h_line, stroke);
    }

    fn find_step(span: f32) -> f32 {
        let min_step = span / 25.0;
        let max_step = span / 10.0;

        let base_steps = [1.0, 2.0, 2.5, 5.0];
        let mut factor = if min_step == 0.0 {
            0.0
        } else {
            10.0_f32.powf(min_step.abs().log10().floor())
        };

        while factor < max_step * 2.0 {
            for &base in &base_steps {
                let step = base * factor;
                if min_step < step && step < max_step {
                    return step;
                }
            }
            factor *= 10.0;
        }

        factor
    }

    fn draw_price_scale(&self, frame: &mut canvas::Frame, window: &Rectangle, bounds: &Rectangle) {
        let step = Self::find_step(window.height);

        let start = (window.y / step).round() as i32;
        let end = ((window.y + window.height) / step).round() as i32;

        for i in start..=end {
            let price = i as f32 * step;

            // let h_start = self.viewport.transform(window.x, price, bounds);
            let h_end = self
                .viewport
                .transform(window.x + window.width, price, bounds);
            //let h_line = Path::line(h_start, h_end);
            //frame.stroke(
            //    &h_line,
            //    Stroke {
            //        width: 1.0,
            //        style: Style::Solid(Color::from_rgb(0.9, 0.1, 0.1)),
            //        ..Stroke::default()
            //    },
            //);

            let text = canvas::Text {
                content: format!("{:.1}", price),
                position: Point::new(h_end.x - 100.0, h_end.y - 8.0),
                size: 16.into(),
                color: Color::from_rgb(0.72, 0.72, 0.72),
                ..Default::default()
            };
            frame.fill_text(text);
        }
    }
}

impl canvas::Program<Message> for CandleChart {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        if let Some(drag_msg) = self.viewport.handle_event(event, bounds, cursor) {
            return Some(Action::publish(Message::Viewport(drag_msg)));
        }

        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let window = self.viewport.get_window(&bounds);

        let chart_geometry = self.chart_cache.draw(renderer, bounds.size(), |frame| {
            let rect = Path::rectangle(bounds.position(), bounds.size());
            frame.fill(&rect, Color::from_rgb(0.06, 0.06, 0.06));

            for (i, candle) in self.data.iter().enumerate() {
                self.draw_candle(frame, candle, i, &bounds);
            }

            self.draw_price_line(frame, &window, &bounds);
        });

        let overlay_geometry = self.overlay_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(cursor_pos) = cursor.position() {
                self.draw_crosshair(frame, cursor_pos, &window, &bounds);
            }

            self.draw_price_scale(frame, &window, &bounds);
        });

        vec![chart_geometry, overlay_geometry]
    }
}
