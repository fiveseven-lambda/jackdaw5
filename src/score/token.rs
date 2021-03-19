use crate::pos::Pos;

pub enum Token<'str> {
    Identifier(&'str str),
    Literal(&'str str),
    Operator(Operator),
}

pub struct TokenPos<'str> {
    pub token: Token<'str>,
    pub pos: Pos,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    ParenOpen,
    ParenClose,
    Semicolon,
    Bar,
    Colon,
    Equal,
    Comma,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
}
