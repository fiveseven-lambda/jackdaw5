use super::token::{Operator, Token, TokenPos};
use crate::pos::Pos;

use std::slice::Iter;

#[derive(Debug)]
pub enum Term<'s, 'p> {
    Identifier(&'s str, &'p Pos),
    Literal(&'s str, &'p Pos),
    Prefix(Prefix, Box<Term<'s, 'p>>),
    Group(Expression<'s, 'p>),
}

#[derive(Debug)]
pub enum Prefix {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Arithmetic {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct Expression<'s, 'p> {
    terms: Vec<(Term<'s, 'p>, Arithmetic)>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError<'s, 'p> {
    #[error("unexpected token ({0:?}) at {1}")]
    UnexpectedToken(&'s str, &'p Pos),
    #[error("unexpected operator ({0:?}) at {1}")]
    UnexpectedOperator(Operator, &'p Pos),
    #[error("parenthesis opened at {0} but not closed until `{1:?}` at {2}")]
    ParenthesisDoesNotMatch(&'p Pos, Operator, &'p Pos),
    #[error("no closing parenthesis to match the opening parenthesis at {0}")]
    NoClosingParenthesis(&'p Pos),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}

pub fn parse_term<'p, 's>(iter: &mut Iter<'p, TokenPos<'s>>) -> Result<Term<'s, 'p>, ParseError<'s, 'p>> {
    match iter.next() {
        Some(TokenPos { token: Token::Identifier(identifier), pos }) => Ok(Term::Identifier(identifier, pos)),
        Some(TokenPos { token: Token::Literal(literal), pos }) => Ok(Term::Literal(literal, pos)),
        Some(TokenPos { token: Token::Operator(operator), pos }) => match operator {
            Operator::ParenOpen => match parse_expression(iter)? {
                (Some((Operator::ParenClose, _)), expression) => Ok(Term::Group(expression)),
                (Some((operator, end)), _) => Err(ParseError::ParenthesisDoesNotMatch(pos, operator, end)),
                (None, _) => Err(ParseError::NoClosingParenthesis(pos)),
            },
            Operator::Plus => Ok(Term::Prefix(Prefix::Add, Box::new(parse_term(iter)?))),
            Operator::Minus => Ok(Term::Prefix(Prefix::Sub, Box::new(parse_term(iter)?))),
            Operator::Asterisk => Ok(Term::Prefix(Prefix::Mul, Box::new(parse_term(iter)?))),
            Operator::Slash => Ok(Term::Prefix(Prefix::Div, Box::new(parse_term(iter)?))),
            &other => Err(ParseError::UnexpectedOperator(other, pos)),
        },
        None => Err(ParseError::UnexpectedEndOfFile),
    }
}

type Delim<'p> = Option<(Operator, &'p Pos)>;

pub fn parse_expression<'p, 's>(iter: &mut Iter<'p, TokenPos<'s>>) -> Result<(Delim<'p>, Expression<'s, 'p>), ParseError<'s, 'p>> {
    let mut ret = Expression { terms: Vec::new() };
    enum State {
        None,
        Division,
        Subtraction,
    }
    let mut state = State::None;
    let (last, delim) = loop {
        let term = parse_term(iter)?;
        let term = match state {
            State::Subtraction => Term::Prefix(Prefix::Sub, Box::new(term)),
            State::Division => Term::Prefix(Prefix::Div, Box::new(term)),
            State::None => term,
        };
        let (next_state, arithmetic) = match iter.next() {
            Some(TokenPos { token: Token::Operator(operator), pos }) => match operator {
                Operator::Plus => (State::None, Arithmetic::Add),
                Operator::Minus => (State::Subtraction, Arithmetic::Add),
                Operator::Asterisk => (State::None, Arithmetic::Mul),
                Operator::Slash => (State::Division, Arithmetic::Mul),
                &other => break (term, Some((other, pos))),
            },
            Some(TokenPos { token: Token::Identifier(token), pos }) | Some(TokenPos { token: Token::Literal(token), pos }) => return Err(ParseError::UnexpectedToken(token, pos)),
            None => break (term, None),
        };
        state = next_state;
        ret.terms.push((term, arithmetic));
    };
    ret.terms.push((last, Arithmetic::Add));
    Ok((delim, ret))
}

use std::collections::HashMap;
use super::compiler::CompileError;
impl<'s, 'p> Term<'s, 'p> {
    pub fn value(&self, variables: &HashMap<&'s str, f64>) -> Result<Option<f64>, CompileError<'s, 'p>> {
        match self {
            &Term::Identifier(s, _) => match s {
                "null" => Ok(None),
                s => Ok(variables.get(s).copied()),
            },
            Term::Literal(s, pos) => match s.parse() {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(CompileError::IllegalLiteral(s, pos, err)),
            },
            Term::Prefix(prefix, term) => Ok(term.value(variables)?.map(|value| match prefix {
                Prefix::Add | Prefix::Mul => value,
                Prefix::Sub => -value,
                Prefix::Div => 1. / value,
            })),
            Term::Group(expression) => expression.value(variables),
        }
    }
}

impl<'s, 'p> Expression<'s, 'p> {
    pub fn value(&self, variables: &HashMap<&'s str, f64>) -> Result<Option<f64>, CompileError<'s, 'p>> {
        let mut x = 0.;
        let mut y = 1.;
        for (term, arithmetic) in &self.terms {
            match term.value(variables)? {
                Some(value) => {
                    y *= value;
                }
                None => return Ok(None),
            }
            match arithmetic {
                Arithmetic::Add => {
                    x += y;
                    y = 1.;
                }
                Arithmetic::Mul => {}
            }
        }
        Ok(Some(x))
    }
}