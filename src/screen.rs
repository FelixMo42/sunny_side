use termion::event::{Key, Event, MouseEvent};
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
            Event::Key(event) => Action::Edit(Edit {
                range: (Spot::new(1, 0), Spot::new(2, 0)),
                text: "~".to_string()
            }),
            Event::Mouse(event) => Action::Quit,
            Event::Unsupported(event) => Action::Quit,
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