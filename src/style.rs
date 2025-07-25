use iced::{Color, Theme};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Color,

    pub bullish: Color,
    pub bearish: Color,

    pub crosshair: Color,

    pub axis_color: Color,

    pub candle_width: f32,
    pub candle_spacing: f32,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &Theme) -> Style {
    Style {
        background: Color::from_rgb(0.06, 0.06, 0.06),
        bullish: Color::from_rgb(0.03, 0.6, 0.5),
        bearish: Color::from_rgb(0.95, 0.21, 0.27),
        crosshair: Color::from_rgb(0.3, 0.3, 0.3),
        axis_color: Color::from_rgb(0.72, 0.72, 0.72),
        candle_width: 4.0,
        candle_spacing: 10.0,
    }
}
