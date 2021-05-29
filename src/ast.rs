use crate::pos::Pos;
use crate::token::Bracket;

// None は空の式を表す
pub type Expression = Option<(Pos, Node)>;

#[derive(Debug)]
pub enum Node {
    Identifier(String, bool),
    Number(String),
    Member(Box<Expression>, String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Invocation(Box<Expression>, Vec<Expression>),
    Group(Bracket, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Nop,
    Minus,
    Reciprocal,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    Greater,
    Equal,
    NotEqual,
    And,
    Or,
    Substitute,
}
