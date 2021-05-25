use crate::ast::{BinaryOperator, Expression, Node, UnaryOperator};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Bracket, Operator, Token, TokenName};

use std::io::BufRead;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Incomplete(Expression, Option<Token>);

impl Incomplete {
    fn map(self, fnc: impl FnOnce(Expression) -> Expression) -> Incomplete {
        Incomplete(fnc(self.0), self.1)
    }
}

fn parse_factor(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    let mut ret = match lexer.next()? {
        Some(Token {
            name: TokenName::Identifier,
            lexeme,
            range,
        }) => (Node::Identifier(lexeme), range),
        Some(Token {
            name: TokenName::Number,
            lexeme,
            range,
        }) => (Node::Number(lexeme), range),
        // 前置の単項演算子
        // 優先順位は関数呼び出しより低い
        Some(Token {
            name: TokenName::Operator(operator @ (Operator::Plus | Operator::Minus | Operator::Asterisk | Operator::Slash | Operator::Exclamation)),
            lexeme: _,
            range: range_operator,
        }) => {
            return parse_factor(lexer).map(|incomplete| {
                incomplete.map(|expr| {
                    let range = match expr {
                        Some((_, ref range)) => range_operator + range.clone(),
                        None => range_operator,
                    };
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
                        range,
                    ))
                })
            });
        }
        // カッコでくくられた部分
        Some(Token {
            name: TokenName::Operator(Operator::Open(open)),
            lexeme: lexeme_open,
            range: range_open,
        }) => match parse_list(lexer)? {
            Incomplete(
                expression,
                Some(Token {
                    name: TokenName::Operator(Operator::Close(close)),
                    lexeme: _,
                    range: range_close,
                }),
            ) if open == close => match open {
                Bracket::Round => (Node::Group(expression.into()), range_open + range_close),
                Bracket::Curly => (Node::Row(expression.into()), range_open + range_close),
                Bracket::Square => (Node::Column(expression.into()), range_open + range_close),
            },
            Incomplete(_, delimiter) => {
                return Err(match delimiter {
                    Some(Token { name: _, lexeme, range }) => Error::UnclosedBraceUntil(lexeme_open, range_open, lexeme, range),
                    None => Error::UnclosedBraceUntilEndOfFile(lexeme_open, range_open),
                }
                .into());
            }
        },
        // パースでは空の式も式として認める
        other => return Ok(Incomplete(None, other)),
    };
    loop {
        match lexer.next()? {
            // 関数呼び出し
            Some(Token {
                name: TokenName::Operator(Operator::Open(Bracket::Round)),
                lexeme: lexeme_open,
                range: range_open,
            }) => match parse_list(lexer)? {
                Incomplete(
                    arg,
                    Some(Token {
                        name: TokenName::Operator(Operator::Close(Bracket::Round)),
                        lexeme: _,
                        range: range_close,
                    }),
                ) => {
                    let range = ret.1.clone() + range_close;
                    ret = (Node::Invocation(Some(ret).into(), arg.into()), range);
                }
                Incomplete(_, delimiter) => {
                    return Err(match delimiter {
                        Some(Token { name: _, lexeme, range }) => Error::UnclosedBraceUntil(lexeme_open, range_open, lexeme, range),
                        None => Error::UnclosedBraceUntilEndOfFile(lexeme_open, range_open),
                    }
                    .into());
                }
            },
            other => {
                return Ok(Incomplete(Some(ret), other));
            }
        }
    }
}
fn parse_list(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
    parse_factor(lexer)
}

macro_rules! def_binary_operator {
    ($prev:ident => $next:ident: $($from:path => $to:expr),*) => {
        fn $next(lexer: &mut Lexer<impl BufRead>) -> Result<Incomplete> {
            let mut ret = $prev(lexer)?;
            loop {
                match ret {
                    $(Incomplete(
                        left,
                        Some(Token {
                            name: TokenName::Operator($from),
                            lexeme: _,
                            range: range_operator,
                        }),
                    ) => {
                        ret = $prev(lexer)?.map(|right| {
                            let range = match left {
                                Some((_, ref left)) => {
                                    left.clone()
                                        + match right {
                                            Some((_, ref right)) => right.clone(),
                                            None => range_operator,
                                        }
                                }
                                None => match right {
                                    Some((_, ref right)) => range_operator + right.clone(),
                                    None => range_operator,
                                },
                            };
                            Some((Node::Binary($to, left.into(), right.into()), range))
                        });
                    })*
                    _ => return Ok(ret),
                }
            }
        }
    }
}
def_binary_operator!(parse_factor => parse_expression1: Operator::Asterisk => BinaryOperator::Mul, Operator::Slash => BinaryOperator::Div);
def_binary_operator!(parse_expression1 => parse_expression2: Operator::Plus => BinaryOperator::Add, Operator::Minus => BinaryOperator::Sub);
def_binary_operator!(parse_expression2 => parse_expression3: Operator::Less => BinaryOperator::Less, Operator::Greater => BinaryOperator::Greater);
def_binary_operator!(parse_expression3 => parse_expression4: Operator::DoubleEqual => BinaryOperator::Equal, Operator::ExclamationEqual => BinaryOperator::NotEqual);
def_binary_operator!(parse_expression4 => parse_score: Operator::DoubleAmpersand => BinaryOperator::And, Operator::DoubleBar => BinaryOperator::Or);
/*

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

*/
