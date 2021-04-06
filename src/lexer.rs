use crate::token::{Operator, Token, TokenPos};

pub struct Lexer<'s> {
    iter: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'s>>>,
}

impl<'s> Lexer<'s> {
    pub fn new(s: &'s str) -> Lexer<'s> {
        Lexer {
            iter: s.chars().enumerate().peekable(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LexError {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, usize),
    #[error("operator `&` at `{0}` (use `&&` instead of `&`)")]
    SingleAmpersand(usize),
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Result<TokenPos, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        let (pos, first) = loop {
            let (i, c) = self.iter.next()?;
            if !c.is_ascii_whitespace() {
                break (i + 1, c);
            }
        };
        Some('get_token: loop {
            break Ok(TokenPos {
                token: match first {
                    '$' => {
                        let mut s = String::new();
                        loop {
                            s.push(match self.iter.peek() {
                                Some(&(_, c)) if c.is_ascii_alphanumeric() || c == '_' => c,
                                _ => break Token::Parameter(s),
                            });
                            self.iter.next();
                        }
                    }
                    'A'..='Z' | 'a'..='z' | '_' => {
                        let mut s = first.to_string();
                        loop {
                            s.push(match self.iter.peek() {
                                Some(&(_, c)) if c.is_ascii_alphanumeric() || c == '_' => c,
                                _ => break Token::Identifier(s),
                            });
                            self.iter.next();
                        }
                    }
                    '0'..='9' | '.' => {
                        let mut s = first.to_string();
                        let mut point = first == '.';
                        loop {
                            s.push(match self.iter.peek() {
                                Some(&(_, c)) if c.is_ascii_digit() => c,
                                Some((_, '.')) if !point => {
                                    point = true;
                                    '.'
                                }
                                _ => break Token::Literal(s),
                            });
                            self.iter.next();
                        }
                    }
                    '+' => Token::Operator(Operator::Plus),
                    '-' => Token::Operator(Operator::Minus),
                    '*' => Token::Operator(Operator::Asterisk),
                    '/' => match self.iter.peek() {
                        Some((_, '/')) => return None,
                        _ => Token::Operator(Operator::Slash),
                    },
                    '<' => Token::Operator(Operator::Less),
                    '>' => Token::Operator(Operator::Greater),
                    '&' => match self.iter.next() {
                        Some((_, '&')) => Token::Operator(Operator::DoubleAmpersand),
                        _ => break 'get_token Err(LexError::SingleAmpersand(pos)),
                    },
                    '|' => match self.iter.peek() {
                        Some((_, '|')) => {
                            self.iter.next();
                            Token::Operator(Operator::DoubleBar)
                        }
                        _ => Token::Operator(Operator::Bar),
                    },
                    '=' => match self.iter.peek() {
                        Some((_, '=')) => {
                            self.iter.next();
                            Token::Operator(Operator::DoubleEqual)
                        }
                        _ => Token::Operator(Operator::Equal),
                    },
                    '!' => match self.iter.peek() {
                        Some((_, '=')) => {
                            self.iter.next();
                            Token::Operator(Operator::NotEqual)
                        }
                        _ => Token::Operator(Operator::Exclamation),
                    },
                    ',' => Token::Operator(Operator::Comma),
                    ':' => Token::Operator(Operator::Colon),
                    ';' => Token::Operator(Operator::Semicolon),
                    '(' => Token::Operator(Operator::ParenOpen),
                    ')' => Token::Operator(Operator::ParenClose),
                    '{' => Token::Operator(Operator::BraceOpen),
                    '}' => Token::Operator(Operator::BraceClose),
                    '[' => Token::Operator(Operator::BracketOpen),
                    ']' => Token::Operator(Operator::BracketClose),
                    _ => break 'get_token Err(LexError::UnexpectedCharacter(first, pos)),
                },
                pos: pos,
            });
        })
    }
}
