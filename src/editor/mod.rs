pub mod cursor;
pub mod document;

use std::cmp::{max, min};
use std::io::Result;

use crate::event::{Event, Key};

use crate::renderer::Screen;
use crate::editor::cursor::Cursor;
use crate::editor::document::{Document, Edit, Spot};
use crate::pain::Pain;

pub struct Editor {
    pub document: Document,
    pub cursor: Cursor,
    pub offset: usize
}

impl Editor {
    pub fn new(source: String) -> Editor {
        return Editor {
            document: Document::new(source),
            offset: 1,
            cursor: Cursor::new()
        }
    }
}

impl Editor {
    fn edit(&mut self, screen: &mut Screen, edit: Option<Edit>) -> Result<()> {
        if let Some(edit) = edit {
            self.document.edit(&edit);

            let mut edited_lines = self.document.get_edit_lines(&edit);

            if edited_lines.1 < self.offset {
                return Ok(());
            }

            edited_lines.0 = max(self.offset, edited_lines.0);

            return self.document.draw(screen, edited_lines, self.offset);
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

            Event::Key(_) => {},

            Event::Mouse(spot) => self.cursor.goto(&self.document, spot),

            Event::Scroll(_, delta) => {
                if delta < 0 {
                    if self.offset > 0 {
                        self.offset -= (-delta) as usize;
                    }
                } else {
                    self.offset += delta as usize;
                }

                self.document.draw(
                    screen,
                    (self.offset, self.offset + screen.size.y),
                    self.offset
                )?
            }
            
            // Other
            Event::Resize(size) => self.document.draw(
                screen,
                (self.offset, self.offset + size.x),
                self.offset
            )?,
        };

        if self.cursor.spot.y >= self.offset {
            return Ok(Spot {
                x: self.cursor.spot.x,
                y: self.cursor.spot.y - self.offset,
            });
        } else {
            return Ok(Spot {
                x: 0,
                y: 0,
            });
        }

        // return Ok(Spot {
        //     x: 0,
        //     y: 0
        // });
    }
}
