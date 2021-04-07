type Iter<'s> = std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'s>>>;

use crate::error::Error;

#[derive(Debug)]
pub enum TokenName {
    Identifier,
    Parameter,
    Literal,
    Operator(Operator),
}

#[derive(Debug)]
pub struct Token {
    name: TokenName,
    lexeme: String,
    pos: usize,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Less,
    Greater,
    NotEqual,
    And,
    Or,
    Not,
    Assign,
    Bar,
    Colon,
    Semicolon,
    Comma,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
}

pub fn next(iter: &mut Iter) -> Option<Result<Token, Error>> {
    let (pos, first) = loop {
        let tmp = iter.next()?;
        if !tmp.1.is_ascii_whitespace() {
            break tmp;
        }
    };
    let mut s = first.to_string();
    Some('get_token: loop {
        break Ok(Token {
            name: match first {
                'A'..='Z' | 'a'..='z' | '_' | '$' => loop {
                    s.push(match iter.peek() {
                        Some(&(_, c)) if c == '_' || c.is_ascii_alphanumeric() => c,
                        _ => {
                            break if first == '$' {
                                TokenName::Parameter
                            } else {
                                TokenName::Identifier
                            }
                        }
                    });
                    iter.next();
                },
                '0'..='9' | '.' => {
                    let mut point = first == '.';
                    loop {
                        s.push(match iter.peek() {
                            Some(&(_, c)) if c.is_ascii_digit() => c,
                            Some(&(_, '.')) if !point => {
                                point = true;
                                '.'
                            }
                            _ => break TokenName::Literal,
                        });
                        iter.next();
                    }
                }
                '+' => TokenName::Operator(Operator::Add),
                '-' => TokenName::Operator(Operator::Sub),
                '*' => TokenName::Operator(Operator::Mul),
                '/' => TokenName::Operator(Operator::Div),
                '=' | '!' => {
                    if let Some(&(_, c @ '=')) = iter.peek() {
                        s.push(c);
                        match first {
                            '=' => TokenName::Operator(Operator::Equal),
                            _ => TokenName::Operator(Operator::NotEqual),
                        }
                    } else {
                        match first {
                            '=' => TokenName::Operator(Operator::Assign),
                            _ => TokenName::Operator(Operator::Not),
                        }
                    }
                }
                '<' => TokenName::Operator(Operator::Less),
                '>' => TokenName::Operator(Operator::Greater),
                '&' => {
                    if let Some(&(_, c @ '&')) = iter.peek() {
                        s.push(c);
                        TokenName::Operator(Operator::And)
                    } else {
                        break 'get_token Err(Error::SingleAmpersand(pos))
                    }
                }
                '|' => {
                    if let Some(&(_, c @ '|')) = iter.peek() {
                        s.push(c);
                        TokenName::Operator(Operator::Or)
                    } else {
                        TokenName::Operator(Operator::Bar)
                    }
                }
                ':' => TokenName::Operator(Operator::Colon),
                ';' => TokenName::Operator(Operator::Semicolon),
                ',' => TokenName::Operator(Operator::Comma),
                '(' => TokenName::Operator(Operator::ParenOpen),
                ')' => TokenName::Operator(Operator::ParenClose),
                '{' => TokenName::Operator(Operator::BraceOpen),
                '}' => TokenName::Operator(Operator::BraceClose),
                '[' => TokenName::Operator(Operator::BracketOpen),
                ']' => TokenName::Operator(Operator::BracketClose),
                other => break 'get_token Err(Error::UnexpectedCharacter(other, pos)),
            },
            lexeme: s,
            pos: pos,
        });
    })
}
