use iced::{
    Element, Event, Length, Point, Rectangle, Renderer, Size, Theme,
    advanced::{
        Clipboard, Layout, Renderer as _, Shell, Widget,
        graphics::geometry::Renderer as _,
        layout::{Limits, Node},
        renderer,
        widget::Tree,
    },
    mouse::Cursor,
    widget::canvas::Cache,
};

use crate::{
    candles::{Candle, generate_data},
    renderer::CandleRenderer,
    style::Catalog,
    viewport::ViewportManager,
};

pub struct CandleChart<Theme>
where
    Theme: Catalog,
{
    width: Length,
    height: Length,
    class: Theme::Class<'static>,

    chart_cache: Cache,
    overlay_cache: Cache,
    viewport: ViewportManager,
    data: Vec<Candle>,
}

impl<Theme> CandleChart<Theme>
where
    Theme: Catalog,
{
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            class: Theme::default(),

            chart_cache: Cache::default(),
            overlay_cache: Cache::default(),
            viewport: ViewportManager::new(),
            data: generate_data(),
        }
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message, Theme> Widget<Message, Theme, Renderer> for CandleChart<Theme>
where
    Message: 'a + Clone,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        Node::new(size)
    }

    fn update(
        &mut self,
        _state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let upd = self.viewport.on_event(event, bounds, cursor);
        if upd {
            self.chart_cache.clear();
            self.overlay_cache.clear();
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        let window = self.viewport.get_window(&bounds);

        let chart_geometry = self.chart_cache.draw(renderer, bounds.size(), |frame| {
            CandleRenderer::draw_chart(frame, &self.viewport, &style, &self.data, &window, &bounds);
        });

        let overlay_geometry = self.overlay_cache.draw(renderer, bounds.size(), |frame| {
            CandleRenderer::draw_overlay(frame, &self.viewport, &style, &cursor, &window, &bounds);
        });

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(chart_geometry);
            renderer.draw_geometry(overlay_geometry);
        });
    }
}

impl<'a, Message, Theme> From<CandleChart<Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: 'a + Catalog,
    Message: Clone + 'a,
{
    fn from(candle_charts: CandleChart<Theme>) -> Self {
        Element::new(candle_charts)
    }
}
