use crate::ast::{BinaryOperator, Expression, Node, UnaryOperator};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::pos::Pos;
use crate::token::{Bracket, Operator, Token, TokenName};

use std::io::BufRead;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// パースした式と，その直後のトークン
type Incomplete = (Expression, Option<Token>);

fn pos(expression: &Expression) -> Option<Pos> {
    expression.as_ref().map(|(pos, _)| pos.clone())
}

fn parse_factor(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    let mut ret = match lexer.next()? {
        Some(Token {
            name: TokenName::Identifier,
            lexeme,
            pos,
        }) => (pos, Node::Identifier(lexeme)),
        Some(Token {
            name: TokenName::Number,
            lexeme,
            pos,
        }) => (pos, Node::Number(lexeme)),
        // 前置の単項演算子
        // 優先順位は関数呼び出しよりも低い
        Some(Token {
            name: TokenName::Operator(operator @ (Operator::Plus | Operator::Minus | Operator::Asterisk | Operator::Slash | Operator::Exclamation)),
            pos: pos_operator,
            ..
        }) => {
            return parse_factor(lexer).map(|(expr, delimiter)| {
                (
                    Some((
                        pos_operator + pos(&expr),
                        Node::Unary(
                            match operator {
                                Operator::Minus => UnaryOperator::Minus,
                                Operator::Slash => UnaryOperator::Reciprocal,
                                Operator::Exclamation => UnaryOperator::Not,
                                _ => UnaryOperator::Nop,
                            },
                            expr.into(),
                        ),
                    )),
                    delimiter,
                )
            })
        }
        // カッコでくくられた部分
        Some(Token {
            name: TokenName::Operator(Operator::Open(open)),
            lexeme: lexeme_open,
            pos: pos_open,
        }) => match parse_list(lexer)? {
            (
                expression,
                Some(Token {
                    name: TokenName::Operator(Operator::Close(close)),
                    pos: pos_close,
                    ..
                }),
            ) if open == close => (pos_open + pos_close, Node::Group(open, expression.into())),
            (_, Some(Token { lexeme, pos, .. })) => return Err(Error::UnclosedBraceUntil(lexeme_open, pos_open, lexeme, pos).into()),
            (_, None) => return Err(Error::UnclosedBraceUntilEndOfFile(lexeme_open, pos_open).into()),
        },
        // パースでは空の式も式として認める
        other => return Ok((None, other)),
    };
    loop {
        match lexer.next()? {
            // 関数呼び出し
            Some(Token {
                name: TokenName::Operator(Operator::Open(Bracket::Round)),
                lexeme: lexeme_open,
                pos: pos_open,
            }) => match parse_list(lexer)? {
                (
                    arg, // 引数
                    Some(Token {
                        name: TokenName::Operator(Operator::Close(Bracket::Round)),
                        pos: pos_close,
                        ..
                    }),
                ) => ret = (ret.0.clone() + pos_close, Node::Invocation(Some(ret).into(), arg.into())),
                (_, Some(Token { lexeme, pos, .. })) => return Err(Error::UnclosedBraceUntil(lexeme_open, pos_open, lexeme, pos).into()),
                (_, None) => return Err(Error::UnclosedBraceUntilEndOfFile(lexeme_open, pos_open).into()),
            },
            // メンバアクセス
            Some(Token {
                name: TokenName::Operator(Operator::Dot),
                ..
            }) => match lexer.next()? {
                Some(Token {
                    name: TokenName::Identifier,
                    lexeme,
                    pos,
                }) => ret = (ret.0.clone() + pos, Node::Member(Some(ret).into(), lexeme)),
                Some(Token { lexeme, pos, .. }) => return Err(Error::UnexpectedToken(lexeme, pos).into()),
                None => return Err(Error::UnexpectedEndOfFile.into()),
            },
            other => return Ok((Some(ret), other)),
        }
    }
}

// 二項演算子の定義
macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),*) => {
        fn $next(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
            let mut ret = $prev(lexer)?;
            loop {
                match ret {
                    $((
                        left,
                        Some(Token {
                            name: TokenName::Operator($from),
                            pos: pos_operator,
                            ..
                        })
                    ) => {
                        let (right, delimiter) = $prev(lexer)?;
                        ret = (
                            Some((pos(&left) + pos_operator + pos(&right), Node::Binary($to, left.into(), right.into()))),
                            delimiter,
                        );
                    }),*
                    _ => return Ok(ret)
                }
            }
        }
    }
}

def_binary_operator!(parse_factor => parse_operator1: Operator::Asterisk => BinaryOperator::Mul, Operator::Slash => BinaryOperator::Div);
def_binary_operator!(parse_operator1 => parse_operator2: Operator::Plus => BinaryOperator::Add, Operator::Minus => BinaryOperator::Sub);
def_binary_operator!(parse_operator2 => parse_operator3: Operator::Less => BinaryOperator::Less, Operator::Greater => BinaryOperator::Greater);
def_binary_operator!(parse_operator3 => parse_operator4: Operator::DoubleEqual => BinaryOperator::Equal, Operator::ExclamationEqual => BinaryOperator::NotEqual);
def_binary_operator!(parse_operator4 => parse_operator5: Operator::DoubleAmpersand => BinaryOperator::And, Operator::DoubleBar => BinaryOperator::Or);

fn parse_map(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    let mut ret = parse_operator5(lexer)?;
    loop {
        match ret {
            (
                left,
                Some(Token {
                    name: TokenName::Operator(Operator::Bar),
                    pos: pos_bar,
                    ..
                }),
            ) => match parse_list(lexer)? {
                (
                    condition,
                    Some(Token {
                        name: TokenName::Operator(Operator::Colon),
                        pos: pos_colon,
                        ..
                    }),
                ) => {
                    let (right, delimiter) = parse_list(lexer)?;
                    ret = (
                        Some((
                            pos(&left) + pos_bar + pos_colon + pos(&right),
                            Node::Map(left.into(), Some(condition.into()), right.into()),
                        )),
                        delimiter,
                    );
                }
                (right, delimiter) => {
                    ret = (
                        Some((pos(&left) + pos_bar + pos(&right), Node::Map(left.into(), None, right.into()))),
                        delimiter,
                    )
                }
            },
            _ => return Ok(ret),
        }
    }
}

fn parse_substitution(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    match parse_map(lexer) {
        Ok((
            left,
            Some(Token {
                name: TokenName::Operator(Operator::Equal),
                pos: pos_operator,
                ..
            }),
        )) => parse_substitution(lexer).map(|(right, delimiter)| {
            (
                Some((
                    pos(&left) + pos_operator + pos(&right),
                    Node::Binary(BinaryOperator::Substitute, left.into(), right.into()),
                )),
                delimiter,
            )
        }),
        other => other,
    }
}

def_binary_operator!(parse_substitution => parse_list: Operator::Comma => BinaryOperator::Comma);

pub fn parse_expression(lexer: &mut Lexer<impl BufRead>) -> Result<Option<Expression>> {
    match parse_list(lexer)? {
        (
            expression,
            Some(Token {
                name: TokenName::Operator(Operator::Semicolon),
                ..
            }),
        ) => Ok(Some(expression)),
        (None, None) => Ok(None),
        (_, Some(Token { lexeme, pos, .. })) => Err(Error::UnexpectedToken(lexeme, pos).into()),
        (_, None) => Err(Error::UnexpectedEndOfFile.into()),
    }
}
