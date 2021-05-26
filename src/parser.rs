use crate::ast::{BinaryOperator, Expression, Node, UnaryOperator};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Bracket, Operator, Token, TokenName};

use std::io::BufRead;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// パースした式と，その直後のトークン
struct Incomplete(Expression, Option<Token>);

fn parse_factor(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    todo!();
}
fn parse_expression(lexer: &mut Lexer<impl BufRead>) -> Result<Expression> {
    todo!();
}
