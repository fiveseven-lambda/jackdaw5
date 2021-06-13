use crate::pos::Pos;

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
    Number(f64),
    String(String), // ←これはエスケープ処理後！
    Member(Box<Expression>, String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Invocation(Box<Expression>, Vec<Expression>),
    Group(Box<Expression>),
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
use crate::function::Argument;
use crate::sound::Sound;
use crate::value::Value;
use std::collections::HashMap;

impl Expression {
    pub fn evaluate(self, variables: &HashMap<String, Value>) -> Option<Result<Value, Error>> {
        self.0.map(|inner| inner.evaluate(variables))
    }
}

macro_rules! eval {
    ($expr:expr, $variables:expr, $pos:expr) => {
        match $expr.evaluate($variables) {
            Some(value) => value?,
            None => return Err(Error::EmptyExpression($pos)),
        }
    };
}

impl PosNode {
    pub fn evaluate(self, variables: &HashMap<String, Value>) -> Result<Value, Error> {
        match self.node {
            Node::Identifier(s, false) => variables.get(&s).map(Clone::clone).ok_or(Error::UndefinedVariable(s, self.pos)),
            Node::Identifier(_, true) => todo!(),
            Node::Number(value) => Ok(Value::Real(value)),
            Node::String(s) => Ok(Value::String(s)),
            Node::Member(_, _) => todo!(),
            Node::Unary(operator, expression) => {
                let value = eval!(expression, variables, self.pos);
                match operator {
                    UnaryOperator::Nop => Ok(value),
                    UnaryOperator::Minus => match value {
                        Value::Real(value) => Ok(Value::Real(-value)),
                        Value::Sound(sound) => Ok(Value::Sound(Sound::Minus(sound.into()))),
                        _ => Err(Error::TypeMismatchMinus(value, self.pos)),
                    },
                    UnaryOperator::Reciprocal => match value {
                        Value::Real(value) => Ok(Value::Real(1. / value)),
                        Value::Sound(sound) => Ok(Value::Sound(Sound::Reciprocal(sound.into()))),
                        _ => Err(Error::TypeMismatchReciprocal(value, self.pos)),
                    },
                    UnaryOperator::Not => match value {
                        Value::Boolean(value) => Ok(Value::Boolean(!value)),
                        _ => Err(Error::TypeMismatchNot(value, self.pos)),
                    },
                }
            }
            Node::Binary(operator, left, right) => match operator {
                BinaryOperator::Add => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left + right)),
                        (Value::Real(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Add(Sound::Const(left).into(), right.into()))),
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(Sound::Add(left.into(), Sound::Const(right).into()))),
                        (Value::Sound(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Add(left.into(), right.into()))),
                        (Value::String(left), Value::String(right)) => Ok(Value::String(left + &right)),
                        (left, right) => return Err(Error::TypeMismatchAdd(left, right, self.pos)),
                    }
                }
                BinaryOperator::Sub => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left - right)),
                        (Value::Real(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Sub(Sound::Const(left).into(), right.into()))),
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(Sound::Sub(left.into(), Sound::Const(right).into()))),
                        (Value::Sound(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Sub(left.into(), right.into()))),
                        (left, right) => return Err(Error::TypeMismatchSub(left, right, self.pos)),
                    }
                }
                BinaryOperator::Mul => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left * right)),
                        (Value::Real(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Mul(Sound::Const(left).into(), right.into()))),
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(Sound::Mul(left.into(), Sound::Const(right).into()))),
                        (Value::Sound(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Mul(left.into(), right.into()))),
                        (left, right) => return Err(Error::TypeMismatchMul(left, right, self.pos)),
                    }
                }
                BinaryOperator::Div => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left / right)),
                        (Value::Real(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Div(Sound::Const(left).into(), right.into()))),
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(Sound::Div(left.into(), Sound::Const(right).into()))),
                        (Value::Sound(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Div(left.into(), right.into()))),
                        (left, right) => return Err(Error::TypeMismatchDiv(left, right, self.pos)),
                    }
                }
                BinaryOperator::Pow => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left.powf(right))),
                        (Value::Real(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Pow(Sound::Const(left).into(), right.into()))),
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(Sound::Pow(left.into(), Sound::Const(right).into()))),
                        (Value::Sound(left), Value::Sound(right)) => Ok(Value::Sound(Sound::Pow(left.into(), right.into()))),
                        (left, right) => return Err(Error::TypeMismatchPow(left, right, self.pos)),
                    }
                }
                BinaryOperator::Less => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left < right)),
                        (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left < right)),
                        (left, right) => return Err(Error::TypeMismatchLess(left, right, self.pos)),
                    }
                }
                BinaryOperator::Greater => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left > right)),
                        (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left > right)),
                        (left, right) => return Err(Error::TypeMismatchGreater(left, right, self.pos)),
                    }
                }
                BinaryOperator::LeftShift => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(left.shift(right))),
                        (left, right) => return Err(Error::TypeMismatchLeftShift(left, right, self.pos)),
                    }
                }
                BinaryOperator::RightShift => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Sound(left), Value::Real(right)) => Ok(Value::Sound(left.shift(-right))),
                        (left, right) => return Err(Error::TypeMismatchRightShift(left, right, self.pos)),
                    }
                }
                BinaryOperator::Equal => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean((left - right).abs() <= 1e-6)),
                        (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left == right)),
                        (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left == right)),
                        (left, right) => return Err(Error::TypeMismatchEqual(left, right, self.pos)),
                    }
                }
                BinaryOperator::NotEqual => {
                    let left = eval!(left, variables, self.pos);
                    let right = eval!(right, variables, self.pos);
                    match (left, right) {
                        (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean((left - right).abs() > 1e-6)),
                        (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left != right)),
                        (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left != right)),
                        (left, right) => return Err(Error::TypeMismatchNotEqual(left, right, self.pos)),
                    }
                }
                BinaryOperator::And => {
                    let left = eval!(left, variables, self.pos);
                    match left {
                        Value::Boolean(true) => {
                            let right = eval!(right, variables, self.pos);
                            match right {
                                Value::Boolean(value) => Ok(Value::Boolean(value)),
                                right => return Err(Error::TypeMismatchAnd2(left, right, self.pos)),
                            }
                        }
                        Value::Boolean(false) => Ok(Value::Boolean(false)),
                        left => return Err(Error::TypeMismatchAnd1(left, self.pos)),
                    }
                }
                BinaryOperator::Or => {
                    let left = eval!(left, variables, self.pos);
                    match left {
                        Value::Boolean(false) => {
                            let right = eval!(right, variables, self.pos);
                            match right {
                                Value::Boolean(value) => Ok(Value::Boolean(value)),
                                right => return Err(Error::TypeMismatchOr2(left, right, self.pos)),
                            }
                        }
                        Value::Boolean(true) => Ok(Value::Boolean(true)),
                        left => return Err(Error::TypeMismatchOr1(left, self.pos)),
                    }
                }
            },
            Node::Invocation(function, arguments) => match function.evaluate(variables) {
                Some(function) => match function? {
                    Value::Function(function) => {
                        let arguments: Vec<_> = arguments
                            .into_iter()
                            .filter_map(|expression| expression.evaluate(variables))
                            .collect::<Result<_, _>>()?;
                        let (vec, map) = function.arguments();
                        if vec.len() != arguments.len() {
                            return Err(Error::WrongNumberOfArguments(vec.len(), arguments.len(), self.pos));
                        }
                        for (i, (cell, value)) in vec.into_iter().zip(arguments).enumerate() {
                            if let Err((type_name, value)) = cell.set(value) {
                                return Err(Error::TypeMismatchArgument(i + 1, type_name, value, self.pos));
                            }
                        }
                        Ok(function.invoke())
                    }
                    Value::RealFunction(function) => {
                        let arguments: Vec<_> = arguments
                            .into_iter()
                            .filter_map(|expression| expression.evaluate(variables))
                            .collect::<Result<_, _>>()?;
                        let (vec, map) = function.arguments();
                        if vec.len() != arguments.len() {
                            return Err(Error::WrongNumberOfArguments(vec.len(), arguments.len(), self.pos));
                        }
                        if vec
                            .iter()
                            .zip(&arguments)
                            .any(|tuple| matches!(tuple, (Argument::Real(_), Value::Sound(_))))
                        {
                            Ok(Value::Sound(Sound::Function(function, arguments, HashMap::new())))
                        } else {
                            for (i, (cell, value)) in vec.into_iter().zip(arguments).enumerate() {
                                if let Err((type_name, value)) = cell.set(value) {
                                    return Err(Error::TypeMismatchArgument(i + 1, type_name, value, self.pos));
                                }
                            }
                            Ok(Value::Real(function.invoke()))
                        }
                    }
                    Value::Sound(sound) => {
                        let arguments: Vec<_> = arguments
                            .into_iter()
                            .filter_map(|expression| expression.evaluate(variables))
                            .collect::<Result<_, _>>()?;
                        match (arguments.get(0), arguments.get(1)) {
                            (Some(Value::String(filename)), Some(Value::Real(time))) => {
                                let samplerate = 44100;
                                let mut iter = sound.iter(samplerate as f64);
                                let spec = hound::WavSpec {
                                    channels: 1,
                                    sample_rate: samplerate,
                                    bits_per_sample: 32,
                                    sample_format: hound::SampleFormat::Int,
                                };
                                let mut writer = hound::WavWriter::create(filename, spec).unwrap();
                                let amplitude = std::i32::MAX as f64;
                                for _ in 0..(time * samplerate as f64) as i64 {
                                    writer.write_sample((amplitude * iter.next()) as i32).unwrap();
                                }
                                writer.finalize().unwrap();
                                Ok(Value::Boolean(true))
                            }
                            _ => {
                                panic!("wrong number of arguments");
                            }
                        }
                    }
                    _ => Err(Error::NotAFunction(self.pos.clone())),
                },
                None => return Err(Error::EmptyExpression(self.pos)),
            },
            Node::Group(expression) => match expression.evaluate(variables) {
                Some(value) => value,
                None => return Err(Error::EmptyExpression(self.pos)),
            },
        }
    }
}
