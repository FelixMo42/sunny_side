mod screen;
mod cursor;
mod document;

use crate::document::Document;
use crate::screen::Ide;

fn main() -> Result<(), std::io::Error> {
    let source = Document {
        source: "let x = 123;\nthis.bla = 23;".to_string()
    };

    return Ide::new()?.run(source);
}