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
}

use crate::error::Error;
use crate::sound::Sound;
use crate::value::Value;

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
                    (_, value) => Err(Error::TypeMismatchUnary(value.typename(), self.pos.clone())),
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
                    (_, left, right) => return Err(Error::TypeMismatchBinary(left.typename(), right.typename(), self.pos.clone())),
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
            Node::Invocation(function, arguments) => {
                let function = function.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))??;
                let arguments = arguments
                    .iter()
                    .filter_map(|expression| expression.0.as_ref().map(|expression| expression.evaluate(variables)))
                    .collect::<Result<_, _>>()?;
                match function {
                    Value::Fnc(function) => match function(arguments) {
                        Some(result) => Ok(result),
                        None => Err(Error::InvocationFailed(self.pos.clone())),
                    },
                    Value::Fnc0(function) => Ok(Value::Real(function())),
                    Value::Fnc1(function) => match &arguments[..] {
                        [Value::Real(x)] => Ok(Value::Real(function(*x))),
                        _ => panic!(),
                    },
                    Value::Fnc2(function) => match &arguments[..] {
                        [Value::Real(x), Value::Real(y)] => Ok(Value::Real(function(*x, *y))),
                        _ => panic!(),
                    },
                    _ => Err(Error::NotAFunction(self.pos.clone())),
                }
            }
            Node::Group(Bracket::Round, expression) => expression.evaluate(variables).ok_or(Error::EmptyExpression(self.pos.clone()))?,
            Node::Group(_, expression) => todo!(),
        }
    }
}
