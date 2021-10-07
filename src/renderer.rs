use termion::event::Key;
use termion::terminal_size;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode};
use termion::cursor::Goto;
use termion::screen::AlternateScreen;

use std::io::{BufWriter, Write, stdout, stdin};

use crate::editor::document::Spot;
use crate::event::Event;
use crate::pain::Pain;

pub struct Screen {
    stream: BufWriter<std::io::Stdout>,
    offset: usize,
}

impl Screen {
    pub fn line(&mut self, y: usize) -> &mut BufWriter<std::io::Stdout> { 
        let y = y + self.offset + 1;
        write!(self.stream, "{}", Goto(1, y as u16));
        return &mut self.stream;
    }
}

pub struct Renderer<T: Pain<Event>> {
    root_pain: T,
}

impl<T: Pain<Event>> Renderer<T> {
    pub fn new(root_pain: T) -> Renderer<T> {
        return Renderer {
            root_pain,
        };
    }
}

impl<T: Pain<Event>> Renderer<T> {
    fn screen(&self) -> Screen {
        return Screen {
            stream: BufWriter::new(stdout()),
            offset: 0,
        }
    }

    fn update(&mut self, event: Event) -> std::io::Result<()> {
        let screen = &mut self.screen();
        let cursor_position = self.root_pain.update(event, screen)?;
        write!(screen.stream, "{}", Goto(
            cursor_position.x as u16 + 1,
            cursor_position.y as u16 + 1,
        ));
        return screen.stream.flush();
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let _stream = stdout().into_raw_mode()?;
        let _stream = AlternateScreen::from(_stream);
        let _stream = MouseTerminal::from(_stream);

        let size = terminal_size()?;
        let size = Spot {
            x: size.0 as usize,
            y: size.1 as usize,
        };

        self.update(Event::Resize(size))?;

        for event in stdin().events() {
            let event = event?;
            let event = Event::from(event);

            if event == Event::Key(Key::Esc) {
                break;
            }

            self.update(event)?;
        }

        return Ok(());
    }
}
