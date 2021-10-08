pub use termion::event::{Key, MouseEvent, MouseButton};

use crate::editor::document::Spot;

#[derive(Eq, PartialEq)]
pub enum Event {
    Key(Key),
    Mouse(Spot),
    Scroll(Spot, isize),
    Resize(Spot),
}

fn to_zero_based(x: u16, y: u16) -> Spot {
    return Spot {
        x: x as usize - 1,
        y: y as usize - 1,
    }
}

impl From<termion::event::Event> for Event {
    fn from(event: termion::event::Event) -> Event {
        match event {
            termion::event::Event::Key(key) => Event::Key(key),

            termion::event::Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Hold(x, y) => Event::Mouse(to_zero_based(x, y)),
                MouseEvent::Release(x, y) => Event::Mouse(to_zero_based(x, y)),
                MouseEvent::Press(button, x, y) => match button {
                    MouseButton::WheelUp => Event::Scroll(to_zero_based(x, y), 1),
                    MouseButton::WheelDown => Event::Scroll(to_zero_based(x, y), -1),
                    _ => Event::Mouse(to_zero_based(x, y)),
                }
            },

            termion::event::Event::Unsupported(_) => Event::Key(Key::Esc)
        }
    }
}