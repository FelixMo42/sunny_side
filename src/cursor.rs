use termion::cursor::Goto;

use crate::document::{Action, Document, Edit, Spot};

pub struct Cursor {
    sticky_x: usize,
    spot: Spot,
}

impl Cursor {
    pub fn new() -> Cursor {
        return Cursor {
            sticky_x: 0,
            spot: Spot {
                x: 0,
                y: 0,
            },
        }
    }
}

impl Cursor {
    pub fn insert(&mut self, chr: char) -> Action {
        let spot = self.spot;

        if chr == '\n' {
            self.spot.x = 0;
            self.spot.y += 1;
        } else {
            self.spot.x += 1;
        }

        self.sticky_x = self.spot.x;

        return Action::Edit(Edit {
            range: (spot, spot),
            text: chr.to_string(),
        });
    }

    pub fn delete(&mut self, document: &Document) -> Action {
        if self.spot.is_zero() {
            return Action::Noop
        }

        let spot = self.spot;

        if self.spot.x == 0 {
            self.spot.y -= 1;
            self.spot.x = document.get_line_length(self.spot.y);
        } else {
            self.spot.x -= 1;
        }

        self.sticky_x = self.spot.x;

        return Action::Edit(Edit {
            range: (self.spot, spot),
            text: "".to_string(),
        });
    }
}

impl Cursor {
    pub fn goto(&mut self, document: &Document, x: usize, y: usize) -> Action {
        self.spot.y = std::cmp::min(document.line_count() - 1, y);
        self.spot.x = std::cmp::min(document.get_line_length(self.spot.y), x);

        return Action::Move;
    }

    pub fn up(&mut self, document: &Document) -> Action {
        if self.spot.y == 0 {
            return Action::Noop;
        }

        self.spot.y -= 1;
        self.spot.x = std::cmp::min(self.sticky_x, document.get_line_length(self.spot.y));

        return Action::Move;
    }

    pub fn down(&mut self, document: &Document) -> Action {
        if self.spot.y == document.line_count() - 1 {
            return Action::Noop;
        }

        self.spot.y += 1;
        self.spot.x = std::cmp::min(self.sticky_x, document.get_line_length(self.spot.y));

        return Action::Move;
    }

    pub fn left(&mut self, document: &Document) -> Action {
        if self.spot.x == 0 {
            if self.spot.y == 0 {
                return Action::Noop;
            }

            self.spot.y -= 1;
            self.spot.x = document.get_line_length(self.spot.y);
        } else {
            self.spot.x -= 1;
        }

        self.sticky_x = self.spot.x;

        return Action::Move;
    }

    pub fn right(&mut self, document: &Document) -> Action {
        if self.spot.x == document.get_line_length(self.spot.y) {
            if self.spot.y == document.line_count() - 1 {
                return Action::Noop;
            }

            self.spot.x = 0;
            self.spot.y += 1;
        } else {
            self.spot.x += 1;
        }

        self.sticky_x = self.spot.x;

        return Action::Move;
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", Goto(
            (self.spot.x + 1) as u16,
            (self.spot.y + 1) as u16,
        ));    
    }
}