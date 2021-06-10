use crate::ast::{BinaryOperator, Expression, Node, UnaryOperator};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Token, TokenName};

use std::io::BufRead;

// パースした式と，その直後のトークン
type Result<T> = std::result::Result<(T, Option<Token>), Box<dyn std::error::Error>>;

fn parse_factor(lexer: &mut Lexer<impl BufRead>) -> Result<Expression> {
    let (mut pos, mut node) = match lexer.next()? {
        Some(Token {
            name: TokenName::Identifier { dollar },
            lexeme,
            pos,
        }) => (pos, Node::Identifier(lexeme, dollar)),
        Some(Token {
            name: TokenName::Number,
            lexeme,
            pos,
        }) => match lexeme.parse() {
            Ok(value) => (pos, Node::Number(value)),
            Err(err) => return Err(Error::FloatParseError(lexeme, pos, err).into()),
        },
        Some(Token {
            name: TokenName::String,
            lexeme,
            pos,
        }) => {
            let mut iter = lexeme.chars().skip(1);
            let mut s = String::new();
            while let Some(c) = iter.next() {
                s.push(match c {
                    '\\' => match iter.next() {
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        Some('0') => '\0',
                        Some(c) => c,
                        None => break,
                    },
                    '"' => break,
                    c => c,
                });
            }
            (pos, Node::String(s))
        }
        // 前置の単項演算子
        // 優先順位は関数呼び出しよりも低い
        Some(Token {
            name: name @ (TokenName::Plus | TokenName::Minus | TokenName::Asterisk | TokenName::Slash | TokenName::Exclamation),
            pos: pos_operator,
            ..
        }) => {
            return parse_factor(lexer).map(|(expr, delimiter)| {
                (
                    Expression::new(
                        pos_operator + expr.pos(),
                        Node::Unary(
                            match name {
                                TokenName::Minus => UnaryOperator::Minus,
                                TokenName::Slash => UnaryOperator::Reciprocal,
                                TokenName::Exclamation => UnaryOperator::Not,
                                _ => UnaryOperator::Nop,
                            },
                            expr.into(),
                        ),
                    ),
                    delimiter,
                )
            })
        }
        // カッコでくくられた部分
        Some(Token {
            name: TokenName::OpeningParen,
            lexeme: lexeme_open,
            pos: pos_open,
        }) => match parse_operator(lexer)? {
            (
                expression,
                Some(Token {
                    name: TokenName::ClosingParen,
                    pos: pos_close,
                    ..
                }),
            ) => (pos_open + pos_close, Node::Group(expression.into())),
            (_, Some(Token { lexeme, pos, .. })) => return Err(Error::UnclosedBraceUntil(lexeme_open, pos_open, lexeme, pos).into()),
            (_, None) => return Err(Error::UnclosedBraceUntilEndOfFile(lexeme_open, pos_open).into()),
        },
        // パースでは空の式も式として認める
        other => return Ok((Expression::empty(), other)),
    };
    loop {
        match lexer.next()? {
            // 関数呼び出し
            Some(Token {
                name: TokenName::OpeningParen,
                lexeme: lexeme_open,
                pos: pos_open,
            }) => match parse_args(lexer)? {
                (
                    arg, // 引数
                    Some(Token {
                        name: TokenName::ClosingParen,
                        pos: pos_close,
                        ..
                    }),
                ) => {
                    node = Node::Invocation(Expression::new(pos.clone(), node).into(), arg);
                    pos = pos + pos_close;
                }
                (_, Some(Token { lexeme, pos, .. })) => return Err(Error::UnclosedBraceUntil(lexeme_open, pos_open, lexeme, pos).into()),
                (_, None) => return Err(Error::UnclosedBraceUntilEndOfFile(lexeme_open, pos_open).into()),
            },
            // メンバアクセス
            Some(Token { name: TokenName::Dot, .. }) => match lexer.next()? {
                Some(Token {
                    name: TokenName::Identifier { .. },
                    lexeme,
                    pos: pos_member,
                }) => {
                    node = Node::Member(Expression::new(pos.clone(), node).into(), lexeme);
                    pos = pos + pos_member;
                }
                Some(Token { lexeme, pos, .. }) => return Err(Error::UnexpectedToken(lexeme, pos).into()),
                None => return Err(Error::UnexpectedEndOfFile.into()),
            },
            other => return Ok((Expression::new(pos, node), other)),
        }
    }
}

// 二項演算子の定義
macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),*) => {
        fn $next(lexer: &mut Lexer<impl BufRead>) -> Result<Expression> {
            let mut ret = $prev(lexer)?;
            loop {
                match ret {
                    $((
                        left,
                        Some(Token {
                            name: $from,
                            pos: pos_operator,
                            ..
                        })
                    ) => {
                        let (right, delimiter) = $prev(lexer)?;
                        ret = (
                            Expression::new(left.pos() + pos_operator + right.pos(), Node::Binary($to, left.into(), right.into())),
                            delimiter,
                        );
                    }),*
                    _ => return Ok(ret)
                }
            }
        }
    }
}

def_binary_operator!(parse_factor => parse_operator1: TokenName::Circumflex => BinaryOperator::Pow);
def_binary_operator!(parse_operator1 => parse_operator2: TokenName::Asterisk => BinaryOperator::Mul, TokenName::Slash => BinaryOperator::Div);
def_binary_operator!(parse_operator2 => parse_operator3: TokenName::Plus => BinaryOperator::Add, TokenName::Minus => BinaryOperator::Sub);
def_binary_operator!(parse_operator3 => parse_operator4: TokenName::DoubleLess => BinaryOperator::LeftShift, TokenName::DoubleGreater => BinaryOperator::RightShift);
def_binary_operator!(parse_operator4 => parse_operator5: TokenName::Less => BinaryOperator::Less, TokenName::Greater => BinaryOperator::Greater);
def_binary_operator!(parse_operator5 => parse_operator6: TokenName::DoubleEqual => BinaryOperator::Equal, TokenName::ExclamationEqual => BinaryOperator::NotEqual);
def_binary_operator!(parse_operator6 => parse_operator: TokenName::DoubleAmpersand => BinaryOperator::And, TokenName::DoubleBar => BinaryOperator::Or);

fn parse_args(lexer: &mut Lexer<impl BufRead>) -> Result<Vec<Expression>> {
    let mut ret = Vec::new();
    loop {
        let (item, delimiter) = parse_operator(lexer)?;
        ret.push(item);
        match delimiter {
            Some(Token { name: TokenName::Comma, .. }) => {}
            other => return Ok((ret, other)),
        }
    }
}

pub fn parse_expression(lexer: &mut Lexer<impl BufRead>) -> std::result::Result<Option<Expression>, Box<dyn std::error::Error>> {
    match parse_operator(lexer)? {
        (
            expression,
            Some(Token {
                name: TokenName::Semicolon, ..
            }),
        ) => Ok(Some(expression)),
        (_, Some(Token { lexeme, pos, .. })) => Err(Error::UnexpectedToken(lexeme, pos).into()),
        (last, None) => {
            if last.is_empty() {
                Ok(None)
            } else {
                Err(Error::UnexpectedEndOfFile.into())
            }
        }
    }
}
