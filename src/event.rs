pub use termion::event::{Key, MouseEvent};

use crate::editor::document::Spot;

#[derive(Eq, PartialEq)]
pub enum Event {
    Key(Key),
    Mouse(Spot),
    Resize(Spot),
}

fn make_mouse_event(x: u16, y: u16) -> Event {
    return Event::Mouse(Spot {
        x: x as usize - 1,
        y: y as usize - 1,
    })
}

impl From<termion::event::Event> for Event {
    fn from(event: termion::event::Event) -> Event {
        match event {
            termion::event::Event::Key(key) => Event::Key(key),
            termion::event::Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Hold(x, y) => make_mouse_event(x, y),
                MouseEvent::Press(_, x, y) => make_mouse_event(x, y),
                MouseEvent::Release(x, y) => make_mouse_event(x, y),
            },
            termion::event::Event::Unsupported(_) => Event::Key(Key::Esc)
        }
    }
}