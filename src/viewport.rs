use iced::{Point, Rectangle, Size, Vector, keyboard, mouse};

use iced::widget::canvas;

#[derive(Debug, Clone, Default)]
pub struct ViewportManager {
    pub offset: Vector,
    scale: f32,
    height: f32,
    mode: i32,
    drag_state: Option<Point>,
}

impl ViewportManager {
    pub fn new() -> Self {
        Self {
            offset: Vector::new(0.0, 117420.0),
            scale: 1.0,
            height: 60000.0,
            mode: 0,
            drag_state: None,
        }
    }

    pub fn on_event(
        &mut self,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> bool {
        match event {
            canvas::Event::Mouse(mouse_event) => self.on_event_mouse(mouse_event, bounds, cursor),
            canvas::Event::Keyboard(keyboard_event) => self.on_event_keyboard(keyboard_event),
            _ => false,
        }
    }

    fn on_event_mouse(
        &mut self,
        event: &mouse::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> bool {
        match event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                self.drag_state = cursor.position_in(bounds);
                self.drag_state.is_some()
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                self.drag_state = None;
                false
            }
            mouse::Event::CursorMoved { position: _ } => {
                if let Some(pos) = cursor.position_in(bounds) {
                    if let Some(last) = self.drag_state {
                        let drag = Vector::new(
                            pos.x - last.x,
                            ((pos.y - last.y) / bounds.height) * self.height * 2.0,
                        );
                        self.offset = self.offset + drag;
                        self.drag_state = Some(pos);
                        return true;
                    }
                }
                false
            }
            mouse::Event::WheelScrolled { delta } => {
                if let Some(pos) = cursor.position_in(bounds) {
                    let zoom_delta = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y,
                        mouse::ScrollDelta::Pixels { y, .. } => *y / 20.0,
                    };

                    if self.mode == 3 {
                        self.height = (self.height + zoom_delta * -5000.0).max(1000.0);
                        return true;
                    } else if self.mode == 2 {
                        self.offset = Vector::new(
                            self.offset.x + 50.0 * zoom_delta * self.scale,
                            self.offset.y,
                        );
                        return true;
                    }

                    let old_scale = self.scale;
                    self.scale = (self.scale + zoom_delta * 0.1).clamp(0.1, 10.0);

                    self.offset = if self.mode == 0 {
                        Vector::new(self.offset.x * (self.scale / old_scale), self.offset.y)
                    } else {
                        let p = pos.x - bounds.width;
                        Vector::new(
                            p + (self.offset.x - p) * (self.scale / old_scale),
                            self.offset.y,
                        )
                    };

                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn on_event_keyboard(&mut self, event: &keyboard::Event) -> bool {
        match event {
            keyboard::Event::KeyPressed { key, .. } => match key {
                keyboard::Key::Named(keyboard::key::Named::Control) => self.mode = 1,
                keyboard::Key::Named(keyboard::key::Named::Shift) => self.mode = 2,
                keyboard::Key::Named(keyboard::key::Named::Alt) => self.mode = 3,
                _ => {}
            },
            keyboard::Event::KeyReleased { key, .. } => match key {
                keyboard::Key::Named(keyboard::key::Named::Control)
                | keyboard::Key::Named(keyboard::key::Named::Shift)
                | keyboard::Key::Named(keyboard::key::Named::Alt) => self.mode = 0,
                _ => {}
            },
            _ => {}
        };
        false
    }

    pub fn transform_point(&self, point: Point, bounds: &Rectangle) -> Point {
        self.transform(point.x, point.y, bounds)
    }

    pub fn transform(&self, x: f32, y: f32, bounds: &Rectangle) -> Point {
        Point::new(
            bounds.width - x * self.scale + self.offset.x,
            (0.5 - (y - self.offset.y) / (self.height * 2.0)) * bounds.height,
        )
    }

    pub fn untransform(&self, x: f32, y: f32, bounds: &Rectangle) -> Point {
        Point::new(
            (x + self.offset.x) / self.scale,
            self.offset.y + (0.5 - y / bounds.height) * (self.height * 2.0),
        )
    }

    pub fn get_window(&self, bounds: &Rectangle) -> Rectangle {
        let pos0 = self.untransform(bounds.width, bounds.height, bounds);
        let pos1 = self.untransform(0.0, 0.0, bounds);

        let size = Size::new(pos1.x - pos0.x, pos1.y - pos0.y);

        Rectangle::new(pos0, size)
    }
}
