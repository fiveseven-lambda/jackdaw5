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
    Equal,
    DoubleEqual,
    Exclamation,
    ExclamationEqual,
    Less,
    Greater,
    DoubleAmpersand,
    DoubleBar,
    Bar,
    Colon,
    Semicolon,
    Comma,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
}
