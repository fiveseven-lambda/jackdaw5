mod reader;
use reader::Reader;

mod slicer;
use slicer::Slicer;

pub struct Lexer<R> {
    reader: Reader<R>,
    slicer: Slicer,
}

use std::io::BufRead;

impl<R> Lexer<R> {
    pub fn new(inner: R, interactive: bool) -> Lexer<R> {
        Lexer {
            reader: Reader::new(inner, interactive),
            slicer: Slicer::from(String::new()),
        }
    }
}

use crate::error::Error;
use crate::pos::Pos;
use crate::token::{Operator, Token, TokenName};

impl<R: BufRead> Iterator for Lexer<R> {
    type Item = Result<Token, Box<dyn std::error::Error>>;
    fn next(&mut self) -> Option<Result<Token, Box<dyn std::error::Error>>> {
        let mut iter = self.slicer.rem().char_indices();
        let (start, first) = loop {
            match iter.next() {
                Some((i, c)) => {
                    if !c.is_ascii_whitespace() {
                        break (i, c);
                    }
                }
                None => match self.reader.read_line() {
                    Ok((0, _)) => return None,
                    Ok((_, s)) => {
                        self.slicer = Slicer::from(s);
                        iter = self.slicer.rem().char_indices();
                    }
                    Err(err) => return Some(Err(Box::new(err))),
                },
            }
        };
        let (end, name) = match first {
            'A'..='Z' | 'a'..='z' | '_' | '$' => loop {
                match iter.next() {
                    Some((i, c)) if !matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '$') => {
                        break (Some(i), TokenName::Identifier)
                    }
                    Some((_, _)) => {}
                    None => break (None, TokenName::Identifier),
                }
            },
            '/' => {
                match iter.next() {
                    Some((_, '/')) => {
                        self.slicer.slice(None, None);
                        return self.next();
                    }
                    Some((_, '*')) => {
                        match self.comment_out() {
                            Ok(true) => return self.next(),
                            Ok(false) => return Some(Err(Box::new(Error::UnterminatedComment))),
                            Err(err) => return Some(Err(Box::new(err)))
                        }
                    }
                    Some((i, _)) => {
                        (Some(i), TokenName::Operator(Operator::Slash))
                    }
                    None => {
                        (None, TokenName::Operator(Operator::Slash))
                    }
                }
            }
            _ => todo!(),
        };
        let pos = Pos::new(self.reader.line(), self.slicer.pos() + start + 1);
        let lexeme = self.slicer.slice(Some(start), end).to_owned();
        Some(Ok(Token {
            name: name,
            lexeme: lexeme,
            pos: pos,
        }))
    }
}

impl<R: BufRead> Lexer<R> {
    fn comment_out(&mut self) -> Result<bool, std::io::Error> {
        let mut iter = self.slicer.rem().char_indices().peekable();
        loop {
            match iter.next() {
                Some((_, '/')) => if let Some((_, '*')) = iter.peek() {
                    iter.next();
                    match iter.next() {
                        Some((index, _)) => self.slicer.slice(None, Some(index)),
                        None => self.slicer.slice(None, None),
                    };
                    self.comment_out()?;
                    iter = self.slicer.rem().char_indices().peekable();
                }
                Some((_, '*')) => if let Some((_, '/')) = iter.peek() {
                    iter.next();
                    match iter.next() {
                        Some((index, _)) => self.slicer.slice(None, Some(index)),
                        None => self.slicer.slice(None, None),
                    };
                    return Ok(true);
                }
                Some((_, _)) => {}
                None => {
                    match self.reader.read_line()? {
                        (0, _) => return Ok(false),
                        (_, s) => {
                            self.slicer = Slicer::from(s);
                            iter = self.slicer.rem().char_indices().peekable();
                        }
                    }
                }
            }
        }
    }
}
