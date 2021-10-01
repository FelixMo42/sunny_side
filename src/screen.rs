use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use std::io::{Write, Stdout, stdout, stdin};

use crate::cursor::Cursor;
use crate::document::{Document, Action};

pub type Screen = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

pub struct Ide {
    screen: Screen,
    cursor: Cursor,
}

impl Ide {
    pub fn new() -> Result<Ide, std::io::Error> {
        let screen = stdout().into_raw_mode()?;
        let screen = AlternateScreen::from(screen);
        let screen = MouseTerminal::from(screen);

        let cursor = Cursor::new();

        return Ok(Ide {
            screen,
            cursor,
        });
    }

    pub fn run(&mut self, mut document: Document) -> Result<(), std::io::Error> {
        write!(self.screen, "{}{}", termion::clear::All, termion::cursor::SteadyBar)?;
        document.display(&mut self.screen)?;
        write!(self.screen, "{}", self.cursor)?;
        self.screen.flush()?;

        for event in stdin().events() {
            let action = match event? {
                // Quit

                Event::Key(Key::Esc) => Action::Quit,

                // Edit

                Event::Key(Key::Backspace) => self.cursor.delete(&document),
                Event::Key(Key::Char(chr)) => self.cursor.insert(chr),

                // Move

                Event::Key(Key::Up)    => self.cursor.up(&document),
                Event::Key(Key::Down)  => self.cursor.down(&document),
                Event::Key(Key::Left)  => self.cursor.left(&document),
                Event::Key(Key::Right) => self.cursor.right(&document),

                Event::Mouse(MouseEvent::Press(_, x, y)) => self.cursor.goto(
                    &document,
                    x as usize - 1,
                    y as usize - 1
                ),

                // Noop

                Event::Key(_) => Action::Noop,
                Event::Mouse(_) => Action::Noop,
                Event::Unsupported(_) => Action::Noop,
            };

            match action {
                Action::Quit => break,
                Action::Noop => {},
                Action::Move => {
                    write!(&mut self.screen, "{}", self.cursor)?;
                    self.screen.flush()?;
                },
                Action::Edit(edit) => {
                    document.edit(&edit);

                    document.display_edit(&mut self.screen, &edit)?;
                    write!(&mut self.screen, "{}", self.cursor)?;
                    self.screen.flush()?;
                }
            }
        }

        self.screen.flush()?;

        return Ok(());
    }
}