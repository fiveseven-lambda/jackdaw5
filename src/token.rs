#[derive(Debug)]
pub struct Pos {
    line: usize,
    pos: usize,
}

#[derive(Debug)]
pub struct TokenPos {
    token: Token,
    pos: Pos,
}

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
}

#[derive(Debug)]
pub enum TokenName {
    Identifier,
    Parameter,
    Literal,
    Operator(Operator),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
    And,
    Or,
    Not,
    Assign,
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
