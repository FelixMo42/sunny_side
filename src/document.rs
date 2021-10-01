use termion::cursor::Goto;
use termion::clear;

use std::io::Write;
use std::str::CharIndices;

use crate::screen::Screen;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Spot {
    pub x: usize,
    pub y: usize,
}

impl Spot {
    pub fn new(x: usize, y: usize) -> Spot {
        return Spot { x, y };
    }
}

pub struct Edit {
    pub text: String,
    pub range: (Spot, Spot),
}

pub struct Document {
    pub source: String,
}

impl Document {
    pub fn display(&self, screen: &mut Screen) -> Result<(), std::io::Error>{
        for (y, line) in self.source.lines().enumerate() {
            self.display_line(screen, y, line)?;
        }

        screen.flush()?;

        return Ok(());
    }

    pub fn display_line(&self, screen: &mut Screen, y: usize, line: &str) -> Result<(), std::io::Error> {
        write!(screen, "{}{}{}",
            Goto(1, (y + 1) as u16),
            line,
            clear::UntilNewline
        )?;

        return Ok(());
    }

    pub fn display_edit(&self, screen: &mut Screen, edit: &Edit) -> Result<(), std::io::Error> {
        let start_line = edit.range.0.y;
        let end_line = {
            let old_lines = edit.range.1.y - edit.range.0.y;
            let new_lines = edit.text.chars().filter(|c| c == &'\n').count();

            if old_lines == new_lines {
                old_lines
            } else {
                self.source.lines().count()
            }
        };

        let changed_lines = self.source.lines().enumerate()
            .skip(start_line)
            .take(end_line + 1);

        for (y, line) in changed_lines {
            self.display_line(screen, y, line)?;
        }

        screen.flush()?;

        return Ok(());
    }
}

fn resolve_spot_with_iter(spot: Spot, current_spot: &mut Spot, chars: &mut CharIndices) -> Option<usize> {
    for (i, chr) in chars {
        let is_at_point = &spot == current_spot;

        if chr == '\n' {
            current_spot.x = 0;
            current_spot.y += 1;
        } else {
            current_spot.x += 1;
        }

        if is_at_point {
            return Some(i);
        }
    }

    return None;
}

impl Document {
    pub fn resolve_range(&self, range: (Spot, Spot)) -> Option<std::ops::Range<usize>> {
        let chars = &mut self.source.char_indices();
        let current_spot = &mut Spot { x: 0, y: 0 };

        if let Some(a) = resolve_spot_with_iter(range.0, current_spot, chars) {
            if range.0 == range.1 {
                return Some(a..a);
            }

            if let Some(b) = resolve_spot_with_iter(range.1, current_spot, chars) {
                return Some(a..b);
            }
        }

        return None;
    }

    pub fn edit(&mut self, edit: &Edit) {
        if let Some(range) = self.resolve_range(edit.range) {
            self.source.replace_range(range, edit.text.as_ref());
        }
    }
}