use crate::pos::Range;

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
    pub range: Range,
}

#[derive(Debug)]
pub enum TokenName {
    Identifier,
    Number,
    Operator(Operator),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleEqual,
    ExclamationEqual,
    Less,
    Greater,
    Ampersand,
    DoubleAmpersand,
    DoubleBar,
    Exclamation,
    Comma,
    Semicolon,
    Equal,
    Colon,
    Bar,
    Open(Bracket),
    Close(Bracket),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bracket {
    Round,
    Curly,
    Square,
}
