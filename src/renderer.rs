use iced::{
    Color, Point, Rectangle, Size,
    mouse::Cursor,
    widget::canvas::{self, LineDash, Path, Stroke},
};

use crate::{candles::Candle, style::Style, viewport::ViewportManager};

pub struct CandleRenderer;

impl CandleRenderer {
    #[inline]
    fn get_candle_color(candle: &Candle, style: &Style) -> Color {
        if candle.close > candle.open {
            style.bullish
        } else {
            style.bearish
        }
    }

    fn draw_candle(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        candle: &Candle,
        index: usize,
        bounds: &Rectangle,
    ) {
        let base_x = index as f32 * style.candle_spacing;
        let color = Self::get_candle_color(candle, style);

        let wick = Path::line(
            viewport.transform(base_x, candle.high, bounds),
            viewport.transform(base_x, candle.low, bounds),
        );

        frame.stroke(&wick, Stroke::default().with_width(2.0).with_color(color));

        let (top, bottom) = if candle.open < candle.close {
            (candle.close, candle.open)
        } else {
            (candle.open, candle.close)
        };

        let opos = viewport.transform(base_x - style.candle_width, top, bounds);
        let cpos = viewport.transform(base_x + style.candle_width, bottom, bounds);

        let body = Path::rectangle(opos, Size::new(cpos.x - opos.x, cpos.y - opos.y));
        frame.fill(&body, color);
    }

    fn draw_price_line(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        candle: &Candle,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        let price = candle.close;
        let color = Self::get_candle_color(candle, style);

        let stroke = Stroke {
            width: 1.0,
            line_dash: canvas::LineDash {
                segments: &[1.0, 2.0],
                offset: 0,
            },
            style: canvas::Style::Solid(color),
            ..Stroke::default()
        };

        let line = Path::line(
            viewport.transform(window.x, price, bounds),
            viewport.transform(window.x + window.width, price, bounds),
        );

        frame.stroke(&line, stroke);
    }

    fn draw_crosshair(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        cursor: Point,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        let stroke = Stroke {
            width: 1.0,
            line_dash: LineDash {
                segments: &[5.0, 6.0],
                offset: 0,
            },
            style: canvas::Style::Solid(style.crosshair),
            ..Stroke::default()
        };

        let mut snap_x = window.x + window.width * (cursor.x / bounds.width);
        snap_x = (snap_x / style.candle_spacing).round() * style.candle_spacing;

        let v_line = Path::line(
            viewport.transform(snap_x, window.y, bounds),
            viewport.transform(snap_x, window.y + window.height, bounds),
        );
        frame.stroke(&v_line, stroke);

        let price = window.y + window.height * (1.0 - cursor.y / bounds.height);
        let h_line = Path::line(
            viewport.transform(window.x, price, bounds),
            viewport.transform(window.x + window.width, price, bounds),
        );
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

    fn draw_price_scale(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        let step = Self::find_step(window.height);

        let start = (window.y / step).round() as i32;
        let end = ((window.y + window.height) / step).round() as i32;

        for i in start..=end {
            let price = i as f32 * step;

            // let h_start = self.viewport.transform(window.x, price, bounds);
            let h_end = viewport.transform(window.x + window.width, price, bounds);
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
                color: style.axis_color,
                ..Default::default()
            };
            frame.fill_text(text);
        }
    }

    pub fn draw_chart(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        candles: &Vec<Candle>,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        if candles.is_empty() {
            return;
        }
        let rect = Path::rectangle(Point::ORIGIN, bounds.size());

        frame.fill(&rect, style.background);

        for (i, candle) in candles.iter().enumerate() {
            Self::draw_candle(frame, viewport, style, candle, i, &bounds);
        }

        Self::draw_price_line(frame, viewport, style, &candles[0], &window, &bounds);
    }

    pub fn draw_overlay(
        frame: &mut canvas::Frame,
        viewport: &ViewportManager,
        style: &Style,
        cursor: &Cursor,
        window: &Rectangle,
        bounds: &Rectangle,
    ) {
        if let Some(cursor_pos) = cursor.position_in(*bounds) {
            Self::draw_crosshair(frame, viewport, style, cursor_pos, &window, &bounds);
        }

        Self::draw_price_scale(frame, viewport, style, &window, &bounds);
    }
}
