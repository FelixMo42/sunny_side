use crate::editor::document::Edit;

pub enum Scope {
    Char,
    Line,
    Word,
    Expr,
    File,
}

pub enum Cursor {
    Next(Scope),
    Prev(Scope),

    Insert(String),
    Delete(Scope),
}

pub enum Editor {
    Cursor(Cursor),
    Document(Edit),

    Save(Option<String>),
    Load(Option<String>),
}
