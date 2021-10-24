use crate::editor::syntax::Synax;
use crate::renderer::Screen;
use crate::Spot;

use std::io::{Result, Write};
use std::str::CharIndices;

use termion::clear;

#[derive(Eq, PartialEq)]
pub struct Edit {
    pub text: String,
    pub range: (Spot, Spot),
}

pub struct Document {
    pub source: String,
}

impl Document {
    pub fn new(source: String) -> Document {
        return Document { source };
    }
}

impl Document {
    pub fn edit(&mut self, edit: &Edit) {
        if let Some(range) = self.resolve_range(edit.range) {
            self.source.replace_range(range, edit.text.as_ref());
        }
    }

    pub fn draw(&self, screen: &mut Screen, lines: (usize, usize), offset: usize) -> Result<()> {
        let changed_lines = self
            .source
            .lines()
            .chain(std::iter::repeat(""))
            .skip(lines.0)
            .take(lines.1 - lines.0)
            .map(|line| Synax::new(line));

        let buffer = screen.line(lines.0 - offset)?;

        const NEXT_LINE: &str = "\x1b[1E";

        for line in changed_lines {
            for (style, token) in line {
                write!(buffer, "{}{}", style, token)?;
            }

            write!(buffer, "{}{}", clear::UntilNewline, NEXT_LINE)?;
        }

        return Ok(());
    }
}

fn resolve_spot_with_iter(
    spot: Spot,
    current_spot: &mut Spot,
    chars: &mut CharIndices,
) -> Option<usize> {
    if spot.is_zero() {
        return Some(0);
    }

    for (i, chr) in chars {
        if chr == '\n' {
            current_spot.x = 0;
            current_spot.y += 1;
        } else {
            current_spot.x += 1;
        }

        if &spot == current_spot {
            return Some(i + 1);
        }
    }

    return None;
}

impl Document {
    pub fn get_line_length(&self, y: usize) -> usize {
        if let Some(line) = self.source.lines().nth(y) {
            return line.len();
        } else {
            return 0;
        }
    }

    pub fn line_count(&self) -> usize {
        return self.source.matches('\n').count() + 1;
    }

    fn resolve_range(&self, range: (Spot, Spot)) -> Option<std::ops::Range<usize>> {
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

    pub fn get_edit_lines(&mut self, edit: &Edit) -> (usize, usize) {
        let start_line = edit.range.0.y;
        let number_of_lines_edited = edit.range.1.y - edit.range.0.y;
        let new_number_of_lines = edit.text.matches('\n').count();

        let end_line = if number_of_lines_edited == new_number_of_lines {
            edit.range.1.y + 1
        } else {
            self.line_count()
        };

        return (start_line, end_line);
    }
}
