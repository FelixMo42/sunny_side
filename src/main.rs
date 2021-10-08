mod renderer;
mod editor;
mod event;
mod pain;

use std::fs::read_to_string;

use crate::editor::Editor;
use crate::renderer::Renderer;

pub use crate::editor::document::Spot;

fn main() -> Result<(), std::io::Error> {
    let source = read_to_string("./Cargo.lock")?;
    let editor = Editor::new(source);

    return Renderer::new(editor).run();
}
