#[derive(Debug)]
pub enum Token {
    Parameter(String),
    Identifier(String),
    Literal(String),
    Operator(Operator),
}

#[derive(Debug)]
pub struct TokenPos {
    pub token: Token,
    pub pos: usize,
}

#[derive(Debug)]
pub enum Operator {
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Less,
    Greater,
    DoubleAmpersand,
    DoubleBar,
    Bar,
    Equal,
    DoubleEqual,
    NotEqual,
    Exclamation,
    Comma,
    Colon,
    Semicolon,
}
