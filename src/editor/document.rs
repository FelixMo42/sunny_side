use termion::clear;

use std::io::{Result, Write};
use std::str::CharIndices;

use crate::renderer::Screen;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Spot {
    pub x: usize,
    pub y: usize,
}

impl Spot {
    pub fn is_zero(&self) -> bool {
        return self.x == 0 && self.y == 0;
    }
}

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
            .enumerate()
            .skip(lines.0)
            .take(lines.1 - lines.0);

        let screen_width = screen.size.x - 6;

        for (y, line) in changed_lines {
            write!(
                screen.line(y - offset)?,
                "{} {}{}",
                format!(
                    "{}{} {:>3} {}{}",
                    termion::color::Bg(termion::color::Cyan),
                    termion::color::Fg(termion::color::Black),
                    y + 1,
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset)
                ),
                if let Some(end) = line.char_indices().nth(screen_width) {
                    &line[..end.0]
                } else {
                    &line[..]
                },
                clear::UntilNewline
            )?;
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
