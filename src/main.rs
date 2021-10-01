mod screen;
mod document;

use crate::document::Document;
use crate::screen::run;

fn main() {
    let source = Document {
        source: "let x = 4;\nlet y = x * 2;".to_string()
    };

    if let Err(error) = run(source) {
        println!("{}", error);
    }
}