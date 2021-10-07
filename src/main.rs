mod renderer;
mod editor;
mod event;
mod pain;

use crate::editor::Editor;
use crate::renderer::Renderer;

pub use crate::editor::document::Spot;

fn main() -> Result<(), std::io::Error> {
    let source = Editor::new("let x = 123;\nthis.bla = 23;\n// noice".to_string());

    return Renderer::new(source).run();
}
