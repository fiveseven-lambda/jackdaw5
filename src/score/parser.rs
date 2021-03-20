use crate::pos::Pos;
use super::token::{Operator, Token, TokenPos};

use std::slice::Iter;

#[derive(Debug)]
pub enum Expression<'s, 'p> {
    Identifier(&'s str, &'p Pos),
    Literal(&'s str, &'p Pos),
    Prefix(Operator, Box<Expression<'s, 'p>>),
    Infix(Operator, Box<Expression<'s, 'p>>, Box<Expression<'s, 'p>>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}

pub fn parse_single_term<'p, 's>(iter: &mut Iter<'p, TokenPos<'s>>) -> Result<Expression<'s, 'p>, ParseError> {
    match iter.next() {
        Some(TokenPos { token: Token::Identifier(identifier), pos }) => Ok(Expression::Identifier(identifier, pos)),
        Some(TokenPos { token: Token::Literal(literal), pos }) => Ok(Expression::Literal(literal, pos)),
        Some(TokenPos { token: Token::Operator(operator), pos }) => todo!(),
        None => Err(ParseError::UnexpectedEndOfFile),
    }
}

pub fn parse_expression<'p, 's>(iter: &mut Iter<'p, TokenPos<'s>>) -> Result<Expression<'s, 'p>, ParseError> {
    todo!();
}