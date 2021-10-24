pub mod cursor;
pub mod document;
pub mod event;
pub mod syntax;

pub use self::cursor::Cursor;
pub use self::document::{Document, Edit};
pub use self::event::Editor as Action;

use crate::pain::Pain;
use crate::renderer::Screen;
use crate::Spot;

use std::cmp::{max, min};
use std::io::Result;

pub struct Editor {
    pub document: Document,
    pub cursor: Cursor,
    pub offset: usize,
}

impl Editor {
    pub fn new(source: String) -> Editor {
        return Editor {
            document: Document::new(source),
            offset: 0,
            cursor: Cursor::new(),
        };
    }
}

impl Pain<Action> for Editor {
    fn update(&mut self, screen: &mut Screen, action: Action) -> Result<Spot> {
        match action {
            Action::Document(edit) => self.document.edit(&edit),
            Action::Cursor(event) => self.cursor.update(event, &mut self.document),
            Action::Resize(size) => self.document.draw(
                screen,
                (self.offset, self.offset + screen.size.y),
                self.offset,
            )?,

            _ => {} //     let mut edited_lines = self.document.get_edit_lines(&edit);

                    //     if edited_lines.1 > self.offset {
                    //         edited_lines.0 = max(edited_lines.0, self.offset);
                    //         edited_lines.1 = min(edited_lines.1, self.offset + screen.size.y);

                    //         self.document.draw(screen, edited_lines, self.offset)?;
                    //     }
                    // }

                    // // Editing
                    // Event::Key(Key::Backspace) => self.delete(screen)?,
                    // Event::Key(Key::Char(chr)) => self.insert(screen, chr)?,

                    // // Cursor movement
                    // Event::Key(Key::Up) => self.cursor.up(&self.document),
                    // Event::Key(Key::Down) => self.cursor.down(&self.document),
                    // Event::Key(Key::Left) => self.cursor.left(&self.document),
                    // Event::Key(Key::Right) => self.cursor.right(&self.document),

                    // Event::Mouse(spot) => self.cursor.goto(
                    //     &self.document,
                    //     Spot {
                    //         x: spot.x,
                    //         y: spot.y + self.offset,
                    //     },
                    // ),

                    // Event::Scroll(_, delta) => {
                    //     if delta < 0 {
                    //         if self.offset > 0 {
                    //             self.offset -= (-delta) as usize;
                    //         }
                    //     } else {
                    //         self.offset += delta as usize;
                    //     }

                    //     self.document.draw(
                    //         screen,
                    //         (self.offset, self.offset + screen.size.y),
                    //         self.offset,
                    //     )?
                    // }
                    // // Other
                    // Event::Resize(_) => self.document.draw(
                    //     screen,
                    //     (self.offset, self.offset + screen.size.y),
                    //     self.offset,
                    // )?,

                    // _ => {}
        };

        if self.cursor.spot.y >= self.offset {
            return Ok(Spot {
                x: self.cursor.spot.x,
                y: self.cursor.spot.y - self.offset,
            });
        } else {
            return Ok(Spot { x: 0, y: 0 });
        }
    }
}
