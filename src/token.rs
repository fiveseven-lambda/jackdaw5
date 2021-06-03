use crate::pos::Pos;

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum TokenName {
    Identifier { dollar: bool },
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
    DoubleLess,
    Greater,
    DoubleGreater,
    Ampersand,
    DoubleAmpersand,
    Bar,
    DoubleBar,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Open(Bracket),
    Close(Bracket),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bracket {
    Round,
    Curly,
    Square,
}
