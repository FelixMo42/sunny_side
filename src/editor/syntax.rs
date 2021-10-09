use logos::{Lexer, Logos};

pub const BLUE: &str = "\x1b[36m";
pub const GREEN: &str = "\x1b[32m";
pub const PURPLE: &str = "\x1b[38;2;208;202;140m";
pub const PINK: &str = "\x1b[38;2;0;200;200m";
pub const BLA: &str = "\x1b[38;2;128;128;0m";

pub const RESET: &str = "\x1b[0m";

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("\".*\"")]
    String,

    #[token("mod")]
    #[token("use")]
    #[token("pub")]
    #[token("fn")]
    #[token("struct")]
    #[token("enum")]
    #[token("const")]
    #[token("let")]
    #[token("if")]
    #[token("else")]
    #[token("match")]
    #[token("return")]
    #[token("impl")]
    KeyWord,

    #[regex(r"[A-Z][a-zA-Z0-9_]*")]
    #[token("usize")]
    #[token("bool")]
    Type,

    #[regex(r"[a-z][a-zA-Z0-9_]*[.:]")]
    Path,

    #[regex(r"[a-z][a-zA-Z0-9_]*")]
    Variable,

    #[regex(r"[-]?[0-9]+[.]?[0-9]?")]
    Number,

    #[regex("[^a-zA-Z0-9_]")]
    Punctuation,

    #[error]
    Error,
}

pub struct Synax<'a> {
    tokens: Lexer<'a, Token>,
}

impl<'a> Synax<'a> {
    pub fn new(text: &'a str) -> Self {
        return Synax {
            tokens: Token::lexer(text),
        };
    }
}

impl<'a> Iterator for Synax<'a> {
    type Item = (&'static str, &'a str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(token) = self.tokens.next() {
            let style = match token {
                Token::Path => RESET,
                Token::String => BLA,
                Token::Variable => PURPLE,
                Token::KeyWord => GREEN,
                Token::Number => BLUE,
                Token::Punctuation => RESET,
                Token::Type => PINK,
                Token::Error => RESET,
            };

            Some((style, self.tokens.slice()))
        } else {
            None
        }
    }
}
