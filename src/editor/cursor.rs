use std::cmp::min;

use crate::editor::document::{Document, Edit, Spot};

pub struct Cursor {
    sticky_x: usize,
    pub spot: Spot,
}

impl Cursor {
    pub fn new() -> Cursor {
        return Cursor {
            sticky_x: 0,
            spot: Spot { x: 0, y: 0 },
        };
    }
}

impl Cursor {
    pub fn insert(&mut self, _document: &Document, chr: char) -> Option<Edit> {
        let spot = self.spot;

        if chr == '\n' {
            self.spot.x = 0;
            self.spot.y += 1;
        } else {
            self.spot.x += 1;
        }

        self.sticky_x = self.spot.x;
        return Some(Edit {
            range: (spot, spot),
            text: chr.to_string(),
        });
    }

    pub fn delete(&mut self, document: &Document) -> Option<Edit> {
        if self.spot.is_zero() {
            return None;
        }

        let spot = self.spot;

        if self.spot.x == 0 {
            self.spot.y -= 1;
            self.spot.x = document.get_line_length(self.spot.y);
        } else {
            self.spot.x -= 1;
        }

        self.sticky_x = self.spot.x;

        return Some(Edit {
            range: (self.spot, spot),
            text: "".to_string(),
        });
    }
}

impl Cursor {
    pub fn goto(&mut self, document: &Document, spot: Spot) {
        self.spot.y = min(document.line_count() - 1, spot.y);
        self.spot.x = min(document.get_line_length(self.spot.y), spot.x);

        self.sticky_x = self.spot.x;
    }
}

impl Cursor {
    pub fn up(&mut self, document: &Document) {
        if self.spot.y == 0 {
            return;
        }

        self.spot.y -= 1;
        self.spot.x = min(self.sticky_x, document.get_line_length(self.spot.y));
    }

    pub fn down(&mut self, document: &Document) {
        if self.spot.y == document.line_count() - 1 {
            return;
        }

        self.spot.y += 1;
        self.spot.x = min(self.sticky_x, document.get_line_length(self.spot.y));
    }

    pub fn left(&mut self, document: &Document) {
        if self.spot.x == 0 {
            if self.spot.y == 0 {
                return;
            }

            self.spot.y -= 1;
            self.spot.x = document.get_line_length(self.spot.y);
        } else {
            self.spot.x -= 1;
        }

        self.sticky_x = self.spot.x;
    }

    pub fn right(&mut self, document: &Document) {
        if self.spot.x == document.get_line_length(self.spot.y) {
            if self.spot.y == document.line_count() - 1 {
                return;
            }

            self.spot.x = 0;
            self.spot.y += 1;
        } else {
            self.spot.x += 1;
        }

        self.sticky_x = self.spot.x;
    }
}
