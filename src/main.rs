mod editor;
mod event;
mod pain;
mod renderer;

use crate::editor::Editor;
use crate::renderer::Renderer;

use std::fs::read_to_string;

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

fn main() -> Result<(), std::io::Error> {
    let source = if let Some(path) = std::env::args().nth(1) {
        read_to_string(path)?
    } else {
        "".to_string()
    };

    let editor = Editor::new(source);

    return Renderer::new(editor).run();
}
