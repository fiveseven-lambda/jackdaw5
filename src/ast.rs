use crate::pos::Pos;
use crate::token::Bracket;

// None は空の式を表す
#[derive(Debug)]
pub struct Expression(Option<(Pos, Node)>);

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

use crate::value::Value;

impl Expression {
    pub fn new(pos: Pos, node: Node) -> Expression {
        Expression(Some((pos, node)))
    }
    pub fn empty() -> Expression {
        Expression(None)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    pub fn pos(&self) -> Option<Pos> {
        self.0.as_ref().map(|(pos, _)| pos.clone())
    }
}
