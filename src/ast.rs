use crate::pos::Pos;

#[derive(Debug)]
pub enum Expression {
    Empty,
    Identifier(String, Pos),
    Number(String, Pos),
    Unary(UnaryOperator, Box<Expression>, Pos),
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
