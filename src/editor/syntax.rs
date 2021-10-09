pub const GREEN: &str = "\x1b[32m";

pub const RESET: &str = "\x1b[0m";

pub struct Synax<'a> {
    // text: &'a str,
    split: Vec<&'a str>,
}

impl<'a> Synax<'a> {
    pub fn new(text: &'a str) -> Self {
        return Synax {
            split: text.split_inclusive(' ').rev().collect(),
        };
    }
}

impl<'a> Iterator for Synax<'a> {
    type Item = (&'static str, &'a str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(word) = self.split.pop() {
            match word {
                "mod " | "impl " | "use " | "fn " | "let " | "pub " | "else " | "if "
                | "return " | "struct " => Some((GREEN, word)),
                word => Some((RESET, word)),
            }
        } else {
            None
        }
    }
}
