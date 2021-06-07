use crate::pos::Pos;
use crate::token::Bracket;

// None は空の式を表す
#[derive(Debug)]
pub struct Expression(Option<PosNode>);

#[derive(Debug)]
pub struct PosNode {
    pos: Pos,
    node: Node,
}

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
    Pow,
    Less,
    Greater,
    LeftShift,
    RightShift,
    Equal,
    NotEqual,
    And,
    Or,
}

impl Expression {
    pub fn new(pos: Pos, node: Node) -> Expression {
        Expression(Some(PosNode { pos: pos, node: node }))
    }
    pub fn empty() -> Expression {
        Expression(None)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    pub fn pos(&self) -> Option<Pos> {
        self.0.as_ref().map(|PosNode { pos, .. }| pos.clone())
    }
    pub fn try_into_identifier(self) -> Option<(String, bool)> {
        match self.0 {
            Some(PosNode {
                node: Node::Identifier(identifier, dollar),
                ..
            }) => Some((identifier, dollar)),
            _ => None,
        }
    }
}

use crate::error::Error;
use crate::sound::Sound;
use crate::value::{Value, ValueCell};

use std::collections::HashMap;

impl Expression {
    pub fn evaluate(&self, variables: &mut HashMap<String, Value>) -> Option<Result<Value, Error>> {
        self.0.as_ref().map(|inner| inner.evaluate(variables))
    }
}

impl PosNode {
    pub fn evaluate(&self, variables: &mut HashMap<String, Value>) -> Result<Value, Error> {
        match &self.node {
            Node::Identifier(ref s, false) => variables
                .get(s)
                .map(Clone::clone)
                .ok_or(Error::UndefinedVariable(s.to_string(), self.pos.clone())),
            Node::Identifier(ref s, true) => todo!(),
            Node::Number(ref s) => match s.parse() {
                Ok(value) => Ok(Value::Real(value)),
                Err(err) => Err(Error::FloatParseError(s.to_string(), self.pos.clone(), err)),
            },
            Node::Member(_, _) => todo!(),
            Node::Unary(operator, expression) => {
                let value = expression.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))??;
                match (operator, value) {
                    (UnaryOperator::Nop, Value::Real(value)) => Ok(Value::Real(value)),
                    (UnaryOperator::Minus, Value::Real(value)) => Ok(Value::Real(-value)),
                    (UnaryOperator::Reciprocal, Value::Real(value)) => Ok(Value::Real(1. / value)),
                    (UnaryOperator::Not, Value::Bool(value)) => Ok(Value::Bool(!value)),
                    (_, value) => Err(Error::TypeMismatch(self.pos.clone())),
                }
            }
            Node::Binary(operator, left, right) => {
                let left = left.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))??;
                let right = right.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))??;
                let (left, right) = match (operator, left, right) {
                    (BinaryOperator::Add, Value::Real(left), Value::Real(right)) => return Ok(Value::Real(left + right)),
                    (BinaryOperator::Sub, Value::Real(left), Value::Real(right)) => return Ok(Value::Real(left - right)),
                    (BinaryOperator::Mul, Value::Real(left), Value::Real(right)) => return Ok(Value::Real(left * right)),
                    (BinaryOperator::Div, Value::Real(left), Value::Real(right)) => return Ok(Value::Real(left / right)),
                    (BinaryOperator::Pow, Value::Real(left), Value::Real(right)) => return Ok(Value::Real(left.powf(right))),
                    (BinaryOperator::Less, Value::Real(left), Value::Real(right)) => return Ok(Value::Bool(left < right)),
                    (BinaryOperator::Greater, Value::Real(left), Value::Real(right)) => return Ok(Value::Bool(left > right)),
                    (BinaryOperator::LeftShift, Value::Sound(left), Value::Real(right)) => return Ok(Value::Sound(left.shift(-right))),
                    (BinaryOperator::RightShift, Value::Sound(left), Value::Real(right)) => return Ok(Value::Sound(left.shift(right))),
                    (BinaryOperator::Equal, Value::Real(left), Value::Real(right)) => return Ok(Value::Bool((left - right).abs() <= 1e-6)),
                    (BinaryOperator::NotEqual, Value::Real(left), Value::Real(right)) => return Ok(Value::Bool((left - right).abs() > 1e-6)),
                    (BinaryOperator::And, Value::Bool(left), Value::Bool(right)) => return Ok(Value::Bool(left && right)),
                    (BinaryOperator::Or, Value::Bool(left), Value::Bool(right)) => return Ok(Value::Bool(left || right)),
                    (_, Value::Real(left), Value::Sound(right)) => (Sound::Const(left), right),
                    (_, Value::Sound(left), Value::Real(right)) => (left, Sound::Const(right)),
                    (_, Value::Sound(left), Value::Sound(right)) => (left, right),
                    (_, left, right) => return Err(Error::TypeMismatch(self.pos.clone())),
                };
                let left = left.into();
                let right = right.into();
                match operator {
                    BinaryOperator::Add => Ok(Value::Sound(Sound::Add(left, right))),
                    BinaryOperator::Sub => Ok(Value::Sound(Sound::Sub(left, right))),
                    BinaryOperator::Mul => Ok(Value::Sound(Sound::Mul(left, right))),
                    BinaryOperator::Div => Ok(Value::Sound(Sound::Div(left, right))),
                    _ => panic!(),
                }
            }
            Node::Invocation(function, arguments) => match function.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))?? {
                Value::Function(function) => {
                    let arguments: Vec<_> = arguments
                        .iter()
                        .filter_map(|expression| expression.0.as_ref().map(|expression| expression.evaluate(variables)))
                        .collect::<Result<_, _>>()?;
                    let cells = function.arguments();
                    if cells.len() != arguments.len() {
                        panic!();
                    }
                    cells.into_iter().zip(arguments.into_iter()).for_each(|tuple| match tuple {
                        (ValueCell::Real(cell), Value::Real(value)) => {
                            cell.replace(value);
                        }
                        (ValueCell::Bool(cell), Value::Bool(value)) => {
                            cell.replace(value);
                        }
                        (ValueCell::Sound(cell), Value::Sound(value)) => {
                            cell.replace(value);
                        }
                        _ => panic!(),
                    });
                    Ok(function.invoke())
                }
                Value::RealFunction(function) => {
                    // todo: RealFunction には Sound も渡せるようにする
                    let arguments: Vec<_> = arguments
                        .iter()
                        .filter_map(|expression| expression.0.as_ref().map(|expression| expression.evaluate(variables)))
                        .collect::<Result<_, _>>()?;
                    let cells = function.arguments();
                    if cells.len() != arguments.len() {
                        panic!();
                    }
                    cells.into_iter().zip(arguments.into_iter()).for_each(|tuple| match tuple {
                        (ValueCell::Real(cell), Value::Real(value)) => {
                            cell.replace(value);
                        }
                        (ValueCell::Bool(cell), Value::Bool(value)) => {
                            cell.replace(value);
                        }
                        (ValueCell::Sound(cell), Value::Sound(value)) => {
                            cell.replace(value);
                        }
                        _ => panic!(),
                    });
                    Ok(Value::Real(function.invoke()))
                }
                _ => Err(Error::NotAFunction(self.pos.clone())),
            },
            Node::Group(Bracket::Round, expression) => expression.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))?,
            Node::Group(_, expression) => todo!(),
        }
    }
}
