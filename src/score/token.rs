use crate::pos::Pos;

#[derive(Debug)]
pub enum Token<'s> {
    Identifier(&'s str),
    Literal(&'s str),
    Operator(Operator),
}

#[derive(Debug)]
pub struct TokenPos<'s> {
    pub token: Token<'s>,
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
