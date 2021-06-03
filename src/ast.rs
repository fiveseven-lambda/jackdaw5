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

use crate::error::Error;
use crate::value::Value;

impl Expression {
    pub fn evaluate(&self) -> Result<Option<Value>, Error> {
        match self.0 {
            None => Ok(None),
            Some((ref pos, ref node)) => match node {
                Node::Number(number) => match number.parse() {
                    Ok(value) => Ok(Some(Value::Real(value))),
                    Err(err) => Err(Error::FloatParseError(number.clone(), pos.clone(), err)),
                },
                Node::Unary(operator, expression) => {
                    let operand = expression.evaluate()?.ok_or(Error::EmptyExpression(pos.clone()))?;
                    match operator {
                        UnaryOperator::Nop => match operand {
                            Value::Real(value) => Ok(Some(Value::Real(value))),
                            Value::Bool(_) => Err(Error::TypeMismatch("real", "bool", pos.clone())),
                        },
                        UnaryOperator::Minus => match operand {
                            Value::Real(value) => Ok(Some(Value::Real(-value))),
                            Value::Bool(_) => Err(Error::TypeMismatch("real", "bool", pos.clone())),
                        },
                        UnaryOperator::Reciprocal => match operand {
                            Value::Real(value) => Ok(Some(Value::Real(1. / value))),
                            Value::Bool(_) => Err(Error::TypeMismatch("real", "bool", pos.clone())),
                        },
                        UnaryOperator::Not => match operand {
                            Value::Real(_) => Err(Error::TypeMismatch("bool", "real", pos.clone())),
                            Value::Bool(value) => Ok(Some(Value::Bool(!value))),
                        },
                    }
                }
                Node::Binary(operator, left, right) => {
                    let left = left.evaluate()?.ok_or(Error::EmptyExpression(pos.clone()))?;
                    let right = right.evaluate()?.ok_or(Error::EmptyExpression(pos.clone()))?;
                    match operator {
                        BinaryOperator::Add => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Real(left + right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Sub => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Real(left - right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Mul => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Real(left * right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Div => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Real(left / right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Less => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Bool(left < right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Greater => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Bool(left > right))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::Equal => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Bool((left - right).abs() <= 1e-6))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::NotEqual => match (left, right) {
                            (Value::Real(left), Value::Real(right)) => Ok(Some(Value::Bool((left - right).abs() > 1e-6))),
                            (_, _) => Err(Error::TypeMismatch("real", "", pos.clone())),
                        },
                        BinaryOperator::And => match (left, right) {
                            (Value::Bool(left), Value::Bool(right)) => Ok(Some(Value::Bool(left && right))),
                            (_, _) => Err(Error::TypeMismatch("bool", "", pos.clone())),
                        },
                        BinaryOperator::Or => match (left, right) {
                            (Value::Bool(left), Value::Bool(right)) => Ok(Some(Value::Bool(left || right))),
                            (_, _) => Err(Error::TypeMismatch("bool", "", pos.clone())),
                        },
                        _ => todo!(),
                    }
                }
                Node::Group(_, expression) => Ok(Some(expression.evaluate()?.ok_or(Error::EmptyExpression(pos.clone()))?)),
                _ => todo!(),
            },
        }
    }
}
