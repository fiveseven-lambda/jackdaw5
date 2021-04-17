use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Operator, Token, TokenName};

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
pub enum Term {
    Identifier(String),
    Parameter(String),
    Literal(String),
    Prefix(Prefix, Box<Term>),
    Group(Expression),
}
#[derive(Debug)]
pub struct Expression {
    terms: Vec<(Term, Arithmetic)>,
}

pub struct Parser<Read, Write> {
    pub lexer: Lexer<Read, Write>,
}
impl<Read, Write> Parser<Read, Write>
where
    Read: std::io::BufRead,
    Write: std::io::Write,
{
    pub fn new(lexer: Lexer<Read, Write>) -> Parser<Read, Write> {
        Parser { lexer: lexer }
    }
    pub fn parse_term(&mut self) -> Result<Term, Box<dyn std::error::Error>> {
        Ok(match self.lexer.next() {
            Some(token) => match token? {
                Token {
                    name: TokenName::Identifier,
                    lexeme,
                    line: _,
                } => Term::Identifier(lexeme),
                Token {
                    name: TokenName::Parameter,
                    lexeme,
                    line: _,
                } => Term::Parameter(lexeme),
                Token {
                    name: TokenName::Literal,
                    lexeme,
                    line: _,
                } => Term::Literal(lexeme),
                Token {
                    name: TokenName::Operator(Operator::Add),
                    lexeme: _,
                    line: _,
                } => Term::Prefix(Prefix::Add, Box::new(self.parse_term()?)),
                Token {
                    name: TokenName::Operator(Operator::Sub),
                    lexeme: _,
                    line: _,
                } => Term::Prefix(Prefix::Sub, Box::new(self.parse_term()?)),
                Token {
                    name: TokenName::Operator(Operator::Mul),
                    lexeme: _,
                    line: _,
                } => Term::Prefix(Prefix::Mul, Box::new(self.parse_term()?)),
                Token {
                    name: TokenName::Operator(Operator::Div),
                    lexeme: _,
                    line: _,
                } => Term::Prefix(Prefix::Div, Box::new(self.parse_term()?)),
                Token {
                    name: TokenName::Operator(_),
                    lexeme,
                    line,
                } => return Err(Box::new(Error::UnexpectedOperator(lexeme, line))),
            },
            None => {
                if self.lexer.read('+')? == 0 {
                    return Err(Box::new(Error::UnexpectedEndOfFile));
                } else {
                    return self.parse_term();
                }
            }
        })
    }
}
