use crate::pos::Pos;

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
    pub pos: Pos,
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
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
}
