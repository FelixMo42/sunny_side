use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use std::io::{Write, Stdout, stdout, stdin};

use crate::document::{Document, Edit, Spot};

pub type Screen = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

enum Action {
    Quit,
    Edit(Edit),
    Noop,
}

pub fn run(mut document: Document) -> Result<(), std::io::Error> {
    let screen = stdout().into_raw_mode()?;
    let screen = AlternateScreen::from(screen);
    let screen = MouseTerminal::from(screen);
    let mut screen = screen;
    let screen = &mut screen;

    write!(screen, "{}", termion::clear::All)?;

    document.display(screen)?;

    for event in stdin().events() {
        let action = match event? {
            Event::Key(Key::Esc) => Action::Quit,
            Event::Key(Key::Backspace) => Action::Edit(Edit {
                range: (Spot::new(1, 0), Spot::new(2, 0)),
                text: "".to_string()
            }),
            Event::Key(Key::Char(c)) => Action::Edit(Edit {
                range: (Spot::new(1, 0), Spot::new(1, 0)),
                text: c.to_string()
            }),

            Event::Key(_) => Action::Noop,
            Event::Mouse(_) => Action::Noop,
            Event::Unsupported(_) => Action::Noop,
        };

        match action {
            Action::Quit => break,
            Action::Noop => {}
            Action::Edit(edit) => {
                document.edit(&edit);
                document.display_edit(screen, &edit)?;
            }
        }
    }

    screen.flush()?;

    return Ok(());
}