use crate::pos::Pos;

pub type Expression = Option<(Node, Pos)>;

#[derive(Debug)]
pub enum Node {
    Identifier(String),
    Number(String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Invocation(Box<Expression>, Box<Expression>),
    Map(Box<Expression>, Option<Box<Expression>>, Box<Expression>),
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
    Comma,
}
