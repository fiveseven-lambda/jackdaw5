use crate::ast::{BinaryOperator, Expression, Node, UnaryOperator};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Bracket, Operator, Token, TokenName};

use std::io::BufRead;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Incomplete {
    expression: Expression,
    delimiter: Option<Token>,
}

impl Incomplete {
    fn map(self, fnc: impl FnOnce(Expression) -> Expression) -> Incomplete {
        Incomplete {
            expression: fnc(self.expression),
            delimiter: self.delimiter,
        }
    }
}

fn parse_factor(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    let mut ret = match lexer.next()? {
        Some(Token {
            name: TokenName::Identifier,
            lexeme,
            pos,
        }) => Some((Node::Identifier(lexeme), pos)),
        Some(Token {
            name: TokenName::Number,
            lexeme,
            pos,
        }) => Some((Node::Number(lexeme), pos)),
        // 前置の単項演算子
        // 優先順位は関数呼び出しより低い
        Some(Token {
            name: TokenName::Operator(operator @ (Operator::Plus | Operator::Minus | Operator::Asterisk | Operator::Slash | Operator::Exclamation)),
            lexeme: _,
            pos,
        }) => {
            return parse_factor(lexer).map(|incomplete| {
                incomplete.map(|expr| {
                    Some((
                        Node::Unary(
                            match operator {
                                Operator::Minus => UnaryOperator::Minus,      // マイナスは負号
                                Operator::Slash => UnaryOperator::Reciprocal, // スラッシュは逆数
                                Operator::Exclamation => UnaryOperator::Not,  // エクスクラメーションマークは否定
                                _ => UnaryOperator::Nop,                      // プラスとアスタリスクは何もしない
                            },
                            expr.into(),
                        ),
                        pos, // 演算子のあった場所
                    ))
                })
            });
        }
        // カッコでくくられた部分
        Some(Token {
            name: TokenName::Operator(Operator::Open(open)),
            lexeme: _,
            pos: pos_open,
        }) => match parse_list(lexer)? {
            Incomplete {
                expression,
                delimiter:
                    Some(Token {
                        name: TokenName::Operator(Operator::Close(close)),
                        ..
                    }),
            } if open == close => match open {
                Bracket::Round => expression,
                Bracket::Curly => Some((Node::Row(expression.into()), pos_open)),
                Bracket::Square => Some((Node::Column(expression.into()), pos_open)),
            },
            Incomplete { expression: _, delimiter } => {
                return Err(match delimiter {
                    Some(Token { name: _, lexeme, pos }) => Error::UnclosedParenthesisUntil(pos_open, lexeme, pos),
                    None => Error::UnclosedParenthesisUntilEndOfFile(pos_open),
                }
                .into());
            }
        },
        // パースでは空の式も式として認める
        other => {
            return Ok(Incomplete {
                expression: None,
                delimiter: other,
            })
        }
    };
    loop {
        match lexer.next()? {
            // 関数呼び出し
            Some(Token {
                name: TokenName::Operator(Operator::Open(Bracket::Round)),
                lexeme: _,
                pos: pos_open,
            }) => match parse_list(lexer)? {
                Incomplete {
                    expression: arg,
                    delimiter:
                        Some(Token {
                            name: TokenName::Operator(Operator::Close(Bracket::Round)),
                            ..
                        }),
                } => ret = Some((Node::Invocation(ret.into(), arg.into()), pos_open)),
                Incomplete { expression: _, delimiter } => {
                    return Err(match delimiter {
                        Some(Token { name: _, lexeme, pos }) => Error::UnclosedParenthesisUntil(pos_open, lexeme, pos),
                        None => Error::UnclosedParenthesisUntilEndOfFile(pos_open),
                    }
                    .into());
                }
            },
            other => {
                return Ok(Incomplete {
                    expression: ret,
                    delimiter: other,
                })
            }
        }
    }
}

// 2 項演算子の定義
macro_rules! def_binary_operator{
    ($prev:ident => $next:ident: $($from:path => $to:expr),*) => {
        fn $next(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
            let mut ret = $prev(lexer)?;
            loop {
                match ret {
                    $(Incomplete {
                        expression: left,
                        delimiter:
                            Some(Token {
                                name: TokenName::Operator($from),
                                lexeme: _,
                                pos,
                            }),
                    } => ret = $prev(lexer)?.map(|right| Some((Node::Binary($to, left.into(), right.into()), pos)))),*,
                    _ => break,
                }
            }
            Ok(ret)
        }
    }
}
def_binary_operator!(parse_factor => parse_expression1: Operator::Asterisk => BinaryOperator::Mul, Operator::Slash => BinaryOperator::Div);
def_binary_operator!(parse_expression1 => parse_expression2: Operator::Plus => BinaryOperator::Add, Operator::Minus => BinaryOperator::Sub);
def_binary_operator!(parse_expression2 => parse_expression3: Operator::Less => BinaryOperator::Less, Operator::Greater => BinaryOperator::Greater);
def_binary_operator!(parse_expression3 => parse_expression4: Operator::DoubleEqual => BinaryOperator::Equal, Operator::ExclamationEqual => BinaryOperator::NotEqual);
def_binary_operator!(parse_expression4 => parse_score: Operator::DoubleAmpersand => BinaryOperator::And, Operator::DoubleBar => BinaryOperator::Or);

fn parse_map(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    let mut ret = parse_score(lexer)?;
    loop {
        match ret {
            Incomplete {
                expression: left,
                delimiter:
                    Some(Token {
                        name: TokenName::Operator(Operator::Bar),
                        lexeme: _,
                        pos,
                    }),
            } => match parse_list(lexer)? {
                Incomplete {
                    expression: condition,
                    delimiter:
                        Some(Token {
                            name: TokenName::Operator(Operator::Colon),
                            ..
                        }),
                } => {
                    ret = parse_list(lexer)?.map(|expression| Some((Node::Map(left.into(), Some(condition.into()), expression.into()), pos)));
                }
                Incomplete { expression, delimiter } => {
                    ret = Incomplete {
                        expression: Some((Node::Map(left.into(), None, expression.into()), pos)),
                        delimiter: delimiter,
                    }
                }
            },
            _ => break,
        }
    }
    Ok(ret)
}

fn parse_substitution(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    match parse_map(lexer) {
        Ok(Incomplete {
            expression: left,
            delimiter: Some(Token {
                name: TokenName::Operator(Operator::Equal),
                pos,
                ..
            }),
        }) => parse_substitution(lexer)
            .map(|incomplete| incomplete.map(|expression| Some((Node::Binary(BinaryOperator::Substitute, left.into(), expression.into()), pos)))),
        other => other,
    }
}

def_binary_operator!(parse_substitution => parse_list: Operator::Comma => BinaryOperator::Comma);

pub fn parse_expression(lexer: &mut Lexer<impl BufRead>) -> Result<Expression> {
    match parse_list(lexer)? {
        Incomplete {
            expression,
            delimiter: Some(Token {
                name: TokenName::Operator(Operator::Semicolon),
                ..
            }),
        } => Ok(expression),
        Incomplete { expression: _, delimiter } => Err(match delimiter {
            Some(Token { name: _, lexeme, pos }) => Error::UnexpectedToken(lexeme, pos),
            None => Error::UnexpectedEndOfFile,
        }
        .into()),
    }
}
