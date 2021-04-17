pub struct Lexer {
    buffer: String,
    cursor: usize,
    line: usize,
    commented: bool,
    peeked: Option<Token>,
}

use crate::error::Error;
use crate::token::{Operator, Token, TokenName};

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            buffer: String::new(),
            cursor: 0,
            line: 0,
            commented: false,
            peeked: None,
        }
    }
    pub fn add(&mut self, string: String) {
        assert!(self.peeked.is_none());
        self.buffer = string;
        self.cursor = 0;
        self.line += 1;
    }
    pub fn peek(&mut self) -> Result<&Option<Token>, Error> {
        if self.peeked.is_none() {
            let rem = &self.buffer[self.cursor..];
            if self.commented {
                let comment_end = "*/";
                match rem.find(comment_end) {
                    Some(index) => {
                        self.cursor += index + comment_end.len();
                        self.commented = false;
                        return self.peek();
                    }
                    None => {
                        self.peeked = None;
                    }
                }
            } else {
                let mut iter = rem.char_indices();
            }
        }
        Ok(&self.peeked)
    }
    pub fn next(&mut self) -> Result<Option<Token>, Error> {
        todo!();
    }
}

/*
pub struct Lexer<Read, Write> {
    input: Read,
    output: Option<Write>,
    buffer: String,
    cursor: usize,
    line: usize,
    comment: bool,
    peeked: Option<Option<Token>>,
}

use crate::error::Error;
use crate::token::{Operator, Token, TokenName};

impl<Read, Write> Lexer<Read, Write>
where
    Read: std::io::BufRead,
    Write: std::io::Write,
{
    pub fn new(input: Read, output: Option<Write>) -> Lexer<Read, Write> {
        Lexer {
            input: input,
            output: output,
            buffer: String::new(),
            cursor: 0,
            line: 0,
            comment: false,
            peeked: None,
        }
    }
    pub fn read(&mut self, prompt: char) -> Result<usize, std::io::Error> {
        if let Some(ref mut output) = self.output {
            write!(output, "{} ", prompt)?;
            output.flush().unwrap();
        }
        self.line += 1;
        self.cursor = 0;
        self.buffer = String::new();
        self.input.read_line(&mut self.buffer)
    }
}

impl<Read, Write> Iterator for Lexer<Read, Write>
where
    Read: std::io::BufRead,
    Write: std::io::Write,
{
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Result<Token, Error>> {
        let rem = &self.buffer[self.cursor..];
        let mut iter = rem.char_indices();
        let index = |tuple| match tuple {
            Some((index, _)) => index,
            None => rem.len(),
        };
        if self.comment {
            let mut prev = '\0';
            loop {
                let next = iter.next()?.1;
                if prev == '*' && next == '/' {
                    self.cursor += index(iter.next());
                    self.comment = false;
                    return self.next();
                } else {
                    prev = next;
                }
            }
        } else {
            let (begin, first) = loop {
                let (i, c) = iter.next()?;
                if !c.is_ascii_whitespace() {
                    break (i, c);
                }
            };
            Some('get_token: loop {
                let (end, name) = match first {
                    'a'..='z' | 'A'..='Z' | '_' | '$' => loop {
                        match iter.next() {
                            Some((_, c)) if c.is_ascii_alphanumeric() || c == '_' => {}
                            other => break (other, if first == '$' { TokenName::Parameter } else { TokenName::Identifier }),
                        }
                    },
                    '0'..='9' | '.' => {
                        let mut point = first == '.';
                        loop {
                            match iter.next() {
                                Some((_, '0'..='9')) => {}
                                Some((_, '.')) if !point => point = true,
                                other => break (other, TokenName::Literal),
                            }
                        }
                    }
                    '+' => (iter.next(), TokenName::Operator(Operator::Add)),
                    '-' => (iter.next(), TokenName::Operator(Operator::Sub)),
                    '*' => (iter.next(), TokenName::Operator(Operator::Mul)),
                    '/' => match iter.next() {
                        Some((_, '*')) => {
                            self.comment = true;
                            self.cursor += index(iter.next());
                            return self.next();
                        }
                        other => (other, TokenName::Operator(Operator::Div)),
                    },
                    '=' => match iter.next() {
                        Some((_, '=')) => (iter.next(), TokenName::Operator(Operator::Equal)),
                        other => (other, TokenName::Operator(Operator::Assign)),
                    },
                    '!' => match iter.next() {
                        Some((_, '=')) => (iter.next(), TokenName::Operator(Operator::NotEqual)),
                        other => (other, TokenName::Operator(Operator::Not)),
                    },
                    '<' => (iter.next(), TokenName::Operator(Operator::Less)),
                    '>' => (iter.next(), TokenName::Operator(Operator::Greater)),
                    '&' => match iter.next() {
                        Some((_, '&')) => (iter.next(), TokenName::Operator(Operator::And)),
                        _ => break 'get_token Err(Error::SingleAmpersand(self.line)),
                    },
                    '|' => match iter.next() {
                        Some((_, '|')) => (iter.next(), TokenName::Operator(Operator::Or)),
                        other => (other, TokenName::Operator(Operator::Bar)),
                    },
                    ':' => (iter.next(), TokenName::Operator(Operator::Colon)),
                    ',' => (iter.next(), TokenName::Operator(Operator::Comma)),
                    ';' => (iter.next(), TokenName::Operator(Operator::Semicolon)),
                    '(' => (iter.next(), TokenName::Operator(Operator::ParenOpen)),
                    ')' => (iter.next(), TokenName::Operator(Operator::ParenClose)),
                    '{' => (iter.next(), TokenName::Operator(Operator::BraceOpen)),
                    '}' => (iter.next(), TokenName::Operator(Operator::BraceClose)),
                    '[' => (iter.next(), TokenName::Operator(Operator::BracketOpen)),
                    ']' => (iter.next(), TokenName::Operator(Operator::BracketClose)),
                    _ => break 'get_token Err(Error::UnexpectedCharacter(first, self.line)),
                };
                let end = index(end);
                self.cursor += end;
                break Ok(Token {
                    name: name,
                    lexeme: rem[begin..end].to_owned(),
                    line: self.line,
                });
            })
        }
    }
}

*/
