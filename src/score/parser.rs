use super::token::{Operator, Token, TokenPos};
use crate::pos::Pos;

use std::slice::Iter;

#[derive(Debug)]
enum Term<'s, 'p> {
    Identifier(&'s str, &'p Pos),
    Literal(&'s str, &'p Pos),
    Prefix(Prefix, Box<Term<'s, 'p>>),
    Group(Expression<'s, 'p>),
}

#[derive(Debug)]
enum Prefix {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Arithmetic {
    Add,
    Mul,
}

#[derive(Debug)]
struct Expression<'s, 'p> {
    terms: Vec<(Term<'s, 'p>, Arithmetic)>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError<'s, 'p> {
    #[error("unexpected token `{0}` at {1}")]
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

fn parse_term<'p, 's>(
    iter: &mut Iter<'p, TokenPos<'s>>,
) -> Result<Term<'s, 'p>, ParseError<'s, 'p>> {
    match iter.next() {
        Some(TokenPos {
            token: Token::Identifier(identifier),
            pos,
        }) => Ok(Term::Identifier(identifier, pos)),
        Some(TokenPos {
            token: Token::Literal(literal),
            pos,
        }) => Ok(Term::Literal(literal, pos)),
        Some(TokenPos {
            token: Token::Operator(operator),
            pos,
        }) => match operator {
            Operator::ParenOpen => match parse_expression(iter)? {
                (Some((Operator::ParenClose, _)), expression) => Ok(Term::Group(expression)),
                (Some((operator, end)), _) => {
                    Err(ParseError::ParenthesisDoesNotMatch(pos, operator, end))
                }
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

fn parse_expression<'s, 'p>(
    iter: &mut Iter<'p, TokenPos<'s>>,
) -> Result<(Delim<'p>, Expression<'s, 'p>), ParseError<'s, 'p>> {
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
            Some(TokenPos {
                token: Token::Operator(operator),
                pos,
            }) => match operator {
                Operator::Plus => (State::None, Arithmetic::Add),
                Operator::Minus => (State::Subtraction, Arithmetic::Add),
                Operator::Asterisk => (State::None, Arithmetic::Mul),
                Operator::Slash => (State::Division, Arithmetic::Mul),
                &other => break (term, Some((other, pos))),
            },
            Some(TokenPos {
                token: Token::Identifier(token),
                pos,
            })
            | Some(TokenPos {
                token: Token::Literal(token),
                pos,
            }) => return Err(ParseError::UnexpectedToken(token, pos)),
            None => break (term, None),
        };
        state = next_state;
        ret.terms.push((term, arithmetic));
    };
    ret.terms.push((last, Arithmetic::Add));
    Ok((delim, ret))
}

#[derive(Debug)]
pub enum Notes<'s, 'p> {
    Note(Option<(&'s str, &'p Pos)>, Option<(&'s str, &'p Pos)>),
    Identifier(&'s str, &'p Pos),
    Row(Vec<Map<'s, 'p>>),
    Column(Vec<Map<'s, 'p>>),
}

#[derive(Debug)]
pub struct Function<'s, 'p> {
    conditions: Vec<(&'s str, Expression<'s, 'p>)>,
    assignments: Vec<(&'s str, Expression<'s, 'p>)>,
}

#[derive(Debug)]
pub struct Map<'s, 'p> {
    pub notes: Notes<'s, 'p>,
    pub functions: Vec<Function<'s, 'p>>,
}

fn parse_maps<'s, 'p>(
    iter: &mut Iter<'p, TokenPos<'s>>,
) -> Result<(Delim<'p>, Vec<Map<'s, 'p>>), ParseError<'s, 'p>> {
    let mut ret = Vec::new();
    while let Some(TokenPos { token, pos }) = iter.next() {
        let (end, notes) = match token {
            &Token::Operator(Operator::BraceOpen) => match parse_maps(iter)? {
                (Some((Operator::BraceClose, _)), score) => (iter.next(), Notes::Row(score)),
                (Some((operator, pos)), _) => {
                    return Err(ParseError::UnexpectedOperator(operator, pos))
                }
                (None, _) => return Err(ParseError::UnexpectedEndOfFile),
            },
            &Token::Operator(Operator::BracketOpen) => match parse_maps(iter)? {
                (Some((Operator::BracketClose, _)), score) => (iter.next(), Notes::Column(score)),
                (Some((operator, pos)), _) => {
                    return Err(ParseError::UnexpectedOperator(operator, pos))
                }
                (None, _) => return Err(ParseError::UnexpectedEndOfFile),
            },
            &Token::Literal(_) | &Token::Operator(Operator::Slash) => {
                let (first, mut slash) = match token {
                    &Token::Literal(literal) => (Some((literal, pos)), false),
                    &Token::Operator(Operator::Slash) => (None, true),
                    _ => unreachable!(),
                };
                let mut second = None;
                loop {
                    match iter.next() {
                        Some(&TokenPos {
                            token: Token::Literal(literal),
                            ref pos,
                        }) => {
                            if slash && second.is_none() {
                                second = Some((literal, pos));
                            } else {
                                return Err(ParseError::UnexpectedToken(literal, pos));
                            }
                        }
                        Some(&TokenPos {
                            token: Token::Operator(Operator::Slash),
                            ref pos,
                        }) => {
                            if !slash {
                                slash = true;
                            } else {
                                return Err(ParseError::UnexpectedOperator(Operator::Slash, pos));
                            }
                        }
                        other => break (other, Notes::Note(first, second)),
                    }
                }
            }
            &Token::Operator(other) => return Err(ParseError::UnexpectedOperator(other, pos)),
            &Token::Identifier(identifier) => (iter.next(), Notes::Identifier(identifier, pos)),
        };
        let mut end = match end {
            Some(TokenPos {
                token: Token::Identifier(token),
                pos,
            })
            | Some(TokenPos {
                token: Token::Literal(token),
                pos,
            }) => return Err(ParseError::UnexpectedToken(token, pos)),
            Some(TokenPos {
                token: Token::Operator(operator),
                pos,
            }) => Some((*operator, pos)),
            None => None,
        };
        let mut functions = Vec::new();
        let delim = loop {
            match end {
                Some((Operator::Bar, _)) => {
                    let mut conditions = Vec::new();
                    loop {
                        let identifier = match iter.next() {
                            Some(&TokenPos {
                                token: Token::Identifier(identifier),
                                pos: _,
                            }) => identifier,
                            Some(&TokenPos {
                                token: Token::Operator(Operator::Colon),
                                pos: _,
                            }) => break,
                            Some(&TokenPos {
                                token: Token::Literal(token),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedToken(token, pos)),
                            Some(&TokenPos {
                                token: Token::Operator(operator),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
                            None => return Err(ParseError::UnexpectedEndOfFile),
                        };
                        match iter.next() {
                            Some(&TokenPos {
                                token: Token::Operator(Operator::Equal),
                                pos: _,
                            }) => {}
                            Some(&TokenPos {
                                token: Token::Identifier(token),
                                ref pos,
                            })
                            | Some(&TokenPos {
                                token: Token::Literal(token),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedToken(token, pos)),
                            Some(&TokenPos {
                                token: Token::Operator(operator),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
                            None => return Err(ParseError::UnexpectedEndOfFile),
                        }
                        let (delim, expression) = parse_expression(iter)?;
                        conditions.push((identifier, expression));
                        match delim {
                            Some((Operator::Colon, _)) => break,
                            Some((Operator::Comma, _)) => {}
                            Some((operator, pos)) => {
                                return Err(ParseError::UnexpectedOperator(operator, pos))
                            }
                            None => return Err(ParseError::UnexpectedEndOfFile),
                        }
                    }
                    let mut assignments = Vec::new();
                    end = loop {
                        let identifier = match iter.next() {
                            Some(&TokenPos {
                                token: Token::Identifier(identifier),
                                pos: _,
                            }) => identifier,
                            Some(&TokenPos {
                                token: Token::Literal(token),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedToken(token, pos)),
                            Some(&TokenPos {
                                token: Token::Operator(operator),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
                            None => return Err(ParseError::UnexpectedEndOfFile),
                        };
                        match iter.next() {
                            Some(TokenPos {
                                token: Token::Operator(Operator::Equal),
                                pos: _,
                            }) => {}
                            Some(&TokenPos {
                                token: Token::Identifier(token),
                                ref pos,
                            })
                            | Some(&TokenPos {
                                token: Token::Literal(token),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedToken(token, pos)),
                            Some(&TokenPos {
                                token: Token::Operator(operator),
                                ref pos,
                            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
                            None => return Err(ParseError::UnexpectedEndOfFile),
                        }
                        let (delim, expression) = parse_expression(iter)?;
                        assignments.push((identifier, expression));
                        match delim {
                            Some((Operator::Colon, _)) => {}
                            other => break other,
                        }
                    };
                    functions.push(Function {
                        conditions: conditions,
                        assignments: assignments,
                    })
                }
                other => break other,
            }
        };
        ret.push(Map {
            notes: notes,
            functions: functions,
        });
        match delim {
            Some((Operator::Comma, _)) => {}
            other => return Ok((other, ret)),
        }
    }
    Ok((None, ret))
}

pub fn parse<'s, 'p>(
    iter: &mut Iter<'p, TokenPos<'s>>,
) -> Result<Vec<(&'s str, Map<'s, 'p>)>, ParseError<'s, 'p>> {
    let mut ret = Vec::new();
    loop {
        let identifier = match iter.next() {
            Some(&TokenPos {
                token: Token::Identifier(identifier),
                pos: _,
            }) => identifier,
            Some(&TokenPos {
                token: Token::Literal(token),
                ref pos,
            }) => return Err(ParseError::UnexpectedToken(token, pos)),
            Some(&TokenPos {
                token: Token::Operator(operator),
                ref pos,
            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
            None => return Err(ParseError::UnexpectedEndOfFile),
        };
        match iter.next() {
            Some(TokenPos {
                token: Token::Operator(Operator::Equal),
                pos: _,
            }) => {}
            Some(&TokenPos {
                token: Token::Identifier(token),
                ref pos,
            })
            | Some(&TokenPos {
                token: Token::Literal(token),
                ref pos,
            }) => return Err(ParseError::UnexpectedToken(token, pos)),
            Some(&TokenPos {
                token: Token::Operator(operator),
                ref pos,
            }) => return Err(ParseError::UnexpectedOperator(operator, pos)),
            None => return Err(ParseError::UnexpectedEndOfFile),
        }
        let (delim, score) = parse_maps(iter)?;
        ret.push((
            identifier,
            Map {
                notes: Notes::Row(score),
                functions: Vec::new(),
            },
        ));
        match delim {
            None => return Ok(ret),
            Some((Operator::Semicolon, _)) => {}
            Some((other, pos)) => return Err(ParseError::UnexpectedOperator(other, pos)),
        }
    }
}

use super::compiler::CompileError;
use std::collections::HashMap;
impl<'s, 'p> Term<'s, 'p> {
    pub fn value(
        &self,
        variables: &HashMap<&'s str, f64>,
    ) -> Result<Option<f64>, CompileError<'s, 'p>> {
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
    pub fn value(
        &self,
        variables: &HashMap<&'s str, f64>,
    ) -> Result<Option<f64>, CompileError<'s, 'p>> {
        let mut x = 0.;
        let mut y = 1.;
        for (term, arithmetic) in &self.terms {
            match term.value(variables)? {
                Some(value) => y *= value,
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

use super::Score;
impl<'s> Score<'s> {
    pub fn map<'p>(&mut self, function: &Function<'s, 'p>) -> Result<(), CompileError<'s, 'p>> {
        match self {
            Score::Note(parameters) => {
                let mut flag = true;
                for (s, expression) in &function.conditions {
                    let left = parameters.get(s);
                    let right = expression.value(parameters)?;
                    flag &= match (left, right) {
                        (None, None) => true,
                        (Some(left), Some(right)) => (left - right).abs() < 1e-5,
                        _ => false,
                    };
                }
                if flag {
                    for (s, expression) in &function.assignments {
                        let right = expression.value(parameters)?;
                        match right {
                            Some(value) => parameters.insert(s, value),
                            None => parameters.remove(s),
                        };
                    }
                }
            }
            Score::Row(scores) | Score::Column(scores) => {
                for score in scores {
                    score.map(function)?;
                }
            }
        };
        Ok(())
    }
}