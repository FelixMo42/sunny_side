pub mod cursor;
pub mod document;

use crate::event::{Event, Key};

use crate::renderer::Screen;
use crate::editor::cursor::Cursor;
use crate::editor::document::{Document, Edit, Spot};
use crate::pain::Pain;

pub struct Editor {
    pub document: Document,
    pub cursor: Cursor,
}

impl Editor {
    pub fn new(source: String) -> Editor {
        return Editor {
            document: Document::new(source),
            cursor: Cursor::new()
        }
    }
}

impl Editor {
    fn edit(&mut self, screen: &mut Screen, edit: Option<Edit>) -> std::io::Result<()> {
        if let Some(edit) = edit {
            self.document.edit(&edit);

            let edited_lines = self.document.get_edit_lines(&edit);

            return self.document.draw(screen, edited_lines);
        } else {
            return Ok(());
        }
    }

    #[inline]
    fn delete(&mut self, screen: &mut Screen) -> std::io::Result<()> {
        let edit = self.cursor.delete(&self.document);
        return self.edit(screen, edit);
    }

    #[inline]
    fn insert(&mut self, screen: &mut Screen, chr: char) -> std::io::Result<()> {
        let edit = self.cursor.insert(&self.document, chr);
        return self.edit(screen, edit);
    }
}

impl Pain<Event> for Editor {
    fn update(&mut self, event: Event, screen: &mut Screen) -> std::io::Result<Spot> {
        match event {
            // Editing
            Event::Key(Key::Backspace) => self.delete(screen)?,
            Event::Key(Key::Char(chr)) => self.insert(screen, chr)?,

            // Cursor movement
            Event::Key(Key::Up)    => self.cursor.up(&self.document),
            Event::Key(Key::Down)  => self.cursor.down(&self.document),
            Event::Key(Key::Left)  => self.cursor.left(&self.document),
            Event::Key(Key::Right) => self.cursor.right(&self.document),

            Event::Mouse(spot) => self.cursor.goto(&self.document, spot),
            
            // Resize
            Event::Resize(size) => self.document.draw(screen, (0, size.x))?,

            _ => {},
        };

        return Ok(self.cursor.spot);
    }
}
