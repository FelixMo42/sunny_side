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
    fn goto(self) -> Goto {
        return Goto(
            (self.x + 1) as u16,
            (self.y + 1) as u16
        );
    }

    fn next_line(self) -> Spot {
        return Spot {
            x: 0,
            y: self.y + 1,
        }
    }
}

pub struct Edit {
    text: String,
    range: (Spot, Spot),
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
            clear::AfterCursor
        )?;

        return Ok(());
    }

    pub fn display_edit(&self, screen: &mut Screen, edit: &Edit) -> Result<(), std::io::Error> {
        let changed_lines = self.source.lines().enumerate()
            .skip(edit.range.0.y)
            .take(edit.range.1.y - edit.range.0.y + 1);

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
    pub fn resolve_spot(&self, spot: Spot) -> Option<usize> {
        return resolve_spot_with_iter(
            spot,
            &mut Spot { x: 0, y: 0 },
            &mut self.source.char_indices(),
        );
    }

    pub fn resolve_range(&self, range: (Spot, Spot)) -> Option<std::ops::RangeInclusive<usize>> {
        let chars = &mut self.source.char_indices();
        let current_spot = &mut Spot { x: 0, y: 0 };

        if let Some(a) = resolve_spot_with_iter(range.0, current_spot, chars) {
            if let Some(b) = resolve_spot_with_iter(range.1, current_spot, chars) {
                return Some(a..=b);
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