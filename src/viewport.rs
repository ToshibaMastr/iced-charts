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

#[derive(Debug, Clone)]
pub enum Message {
    MousePressed(Point),
    MouseReleased,
    MouseMoved(Point, Rectangle),

    Mode(i32),

    Zoom(f32, Point, Rectangle),

    Reset,
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

    pub fn update(&mut self, message: Message) -> bool {
        match message {
            Message::MousePressed(pos) => {
                self.drag_state = Some(pos);
                true
            }
            Message::MouseReleased => {
                self.drag_state = None;
                false
            }
            Message::MouseMoved(pos, bounds) => {
                if let Some(last) = self.drag_state {
                    let dx = pos.x - last.x;
                    let dy = ((pos.y - last.y) / bounds.height) * self.height * 2.0;
                    self.offset = self.offset + Vector::new(dx, dy);
                    self.drag_state = Some(pos);
                }
                true
            }
            Message::Zoom(delta, center, bounds) => {
                if self.mode == 3 {
                    self.height = (self.height + delta * 5000.0).max(1000.0);

                    return true;
                }

                if self.mode == 2 {
                    self.offset =
                        Vector::new(self.offset.x + 50.0 * delta * self.scale, self.offset.y);
                    return true;
                }

                let old_scale = self.scale;
                self.scale = (self.scale + delta * 0.1).clamp(0.1, 10.0);

                self.offset = if self.mode == 0 {
                    Vector::new(self.offset.x * (self.scale / old_scale), self.offset.y)
                } else {
                    let p = center.x - bounds.width;
                    Vector::new(
                        p + (self.offset.x - p) * (self.scale / old_scale),
                        self.offset.y,
                    )
                };

                true
            }
            Message::Mode(mode) => {
                self.mode = mode;
                false
            }
            Message::Reset => {
                self.offset = Vector::new(0.0, 0.0);
                true
            }
        }
    }

    pub fn handle_event(
        &self,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Message> {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event, bounds, cursor)
            }
            canvas::Event::Keyboard(keyboard_event) => self.handle_keyboard_event(keyboard_event),
            _ => None,
        }
    }

    fn handle_mouse_event(
        &self,
        event: &mouse::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Message> {
        match event {
            mouse::Event::ButtonPressed(mouse::Button::Left) => {
                cursor.position_in(bounds).map(Message::MousePressed)
            }
            mouse::Event::ButtonReleased(mouse::Button::Left) => {
                Some(Message::MouseReleased)
            },
            mouse::Event::CursorMoved { position } => {
                Some(Message::MouseMoved(*position, bounds))
            }
            mouse::Event::WheelScrolled { delta } => cursor.position_in(bounds).map(|position| {
                let zoom_delta = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => *y,
                    mouse::ScrollDelta::Pixels { y, .. } => *y / 20.0,
                };
                Message::Zoom(zoom_delta, position, bounds)
            }),
            _ => None,
        }
    }

    fn handle_keyboard_event(&self, event: &keyboard::Event) -> Option<Message> {
        match event {
            keyboard::Event::KeyPressed { key, .. } => match key {
                keyboard::Key::Named(keyboard::key::Named::Control) => Some(Message::Mode(1)),
                keyboard::Key::Named(keyboard::key::Named::Shift) => Some(Message::Mode(2)),
                keyboard::Key::Named(keyboard::key::Named::Alt) => Some(Message::Mode(3)),
                _ => None,
            },
            keyboard::Event::KeyReleased { key, .. } => match key {
                keyboard::Key::Named(keyboard::key::Named::Control)
                | keyboard::Key::Named(keyboard::key::Named::Shift)
                | keyboard::Key::Named(keyboard::key::Named::Alt) => Some(Message::Mode(0)),
                _ => None,
            },
            _ => None,
        }
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
