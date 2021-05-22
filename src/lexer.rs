use crate::token::{Operator, Token, TokenName};
use std::collections::VecDeque;

use crate::error::Error;
use crate::pos::Pos;

pub struct Lexer<BufRead> {
    reader: BufRead,
    prompt: bool,
    queue: VecDeque<Token>,
    line: usize,
    comment: Vec<Pos>,
}

impl<BufRead: std::io::BufRead> Lexer<BufRead> {
    pub fn new(reader: BufRead, prompt: bool) -> Lexer<BufRead> {
        Lexer {
            reader: reader,
            prompt: prompt,
            queue: VecDeque::new(),
            line: 0,
            comment: Vec::new(),
        }
    }
    fn pos(&self, pos: usize) -> Pos {
        Pos::new(self.line, pos + 1)
    }
    pub fn read_line(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.prompt {
            use std::io::Write;
            print!("> ");
            std::io::stdout().flush()?;
        }
        let mut s = String::new();
        if self.reader.read_line(&mut s)? == 0 {
            match self.comment.pop() {
                Some(pos) => Err(Error::UnterminatedComment(pos).into()),
                None => Ok(false),
            }
        } else {
            self.line += 1;

            enum State {
                Initial,
                Identifier,
                Number(bool),
                Operator(Operator),
            }

            let mut prev = State::Initial;
            let mut prev_index = 0;
            let mut prev_pos = 0;

            let mut iter = s.char_indices().enumerate().peekable();

            'entire: while let Some((pos, (index, c))) = iter.next() {
                if self.comment.len() > 0 {
                    match c {
                        '*' => {
                            if let Some((_, (_, '/'))) = iter.peek() {
                                iter.next();
                                self.comment.pop();
                            }
                        }
                        '/' => {
                            if let Some((_, (_, '*'))) = iter.peek() {
                                iter.next();
                                self.comment.push(Pos::new(self.line, pos));
                            }
                        }
                        _ => {}
                    }
                } else {
                    let next = {
                        match c {
                            'A'..='Z' | 'a'..='z' | '_' | '$' => State::Identifier,
                            '0'..='9' => State::Number(false),
                            '.' => State::Number(true),
                            c if c.is_ascii_whitespace() => State::Initial,
                            _ => State::Operator(match c {
                                '+' => Operator::Plus,
                                '-' => Operator::Minus,
                                '*' => Operator::Asterisk,
                                '/' => Operator::Slash,
                                '=' => Operator::Equal,
                                '!' => Operator::Exclamation,
                                '<' => Operator::Less,
                                '>' => Operator::Greater,
                                '&' => Operator::Ampersand,
                                '|' => Operator::Bar,
                                ':' => Operator::Colon,
                                ';' => Operator::Semicolon,
                                ',' => Operator::Comma,
                                '(' => Operator::ParenOpen,
                                ')' => Operator::ParenClose,
                                '{' => Operator::BraceOpen,
                                '}' => Operator::BraceClose,
                                '[' => Operator::BracketOpen,
                                ']' => Operator::BracketClose,
                                _ => {
                                    return Err(Error::UnexpectedCharacter(c, self.pos(pos)).into())
                                }
                            }),
                        }
                    };
                    'token: loop {
                        self.queue.push_back(Token {
                            name: match prev {
                                State::Initial => break 'token,
                                State::Identifier => match next {
                                    State::Identifier | State::Number(false) => continue 'entire,
                                    _ => TokenName::Identifier,
                                },
                                State::Number(prev_point) => match next {
                                    State::Number(point) if !(prev_point && point) => {
                                        prev = State::Number(prev_point || point);
                                        continue 'entire;
                                    }
                                    _ => TokenName::Number,
                                },
                                State::Operator(Operator::Slash) => match next {
                                    // これ以降はラインコメントなのでループを抜ける
                                    State::Operator(Operator::Slash) => break 'entire,
                                    // ブロックコメントの開始
                                    State::Operator(Operator::Asterisk) => {
                                        self.comment.push(self.pos(prev_pos));
                                        continue 'entire;
                                    }
                                    _ => TokenName::Operator(Operator::Slash),
                                },
                                State::Operator(Operator::Exclamation)
                                    if matches!(next, State::Operator(Operator::Equal)) =>
                                {
                                    prev = State::Operator(Operator::ExclamationEqual);
                                    continue 'entire;
                                }
                                State::Operator(Operator::Equal)
                                    if matches!(next, State::Operator(Operator::Equal)) =>
                                {
                                    prev = State::Operator(Operator::DoubleEqual);
                                    continue 'entire;
                                }
                                State::Operator(Operator::Ampersand)
                                    if matches!(next, State::Operator(Operator::Ampersand)) =>
                                {
                                    prev = State::Operator(Operator::DoubleAmpersand);
                                    continue 'entire;
                                }
                                State::Operator(Operator::Bar)
                                    if matches!(next, State::Operator(Operator::Bar)) =>
                                {
                                    prev = State::Operator(Operator::DoubleBar);
                                    continue 'entire;
                                }
                                State::Operator(operator) => TokenName::Operator(operator),
                            },
                            lexeme: s[prev_index..index].to_string(),
                            pos: self.pos(prev_pos),
                        });
                        break;
                    }
                    prev = next;
                    prev_index = index;
                    prev_pos = pos;
                }
            }

            Ok(true)
        }
    }
}

impl<BufRead: std::io::BufRead> Iterator for Lexer<BufRead> {
    type Item = Result<Token, Box<dyn std::error::Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(token) => Some(Ok(token)),
            None => match self.read_line() {
                Ok(true) => self.next(),
                Ok(false) => None,
                Err(err) => Some(Err(err)),
            },
        }
    }
}

#[test]
fn test_lexer() {
    let input: &[u8] = b"
    hoge    fuga
    100    3.14
    xx11__$$
    15abc  abc0.5
    .5  12.  1.2.3.
    white \t \r\n \x0C space
    hoge // line comment
    1 /* block
    comment */ 2 //* line comment
    /* nested
    /*/ block
    /* comment **/*/**// slash
    operators: 
    + - * /
    == != < >
    ! && ||
    | : ; ,
    ( ) { } [ ]
    ";

    let tokens: Vec<_> = Lexer::new(input, false).collect::<Result<_, _>>().unwrap();
    let lexemes: Vec<_> = tokens
        .into_iter()
        .map(|Token { lexeme, .. }| lexeme)
        .collect();

    assert_eq!(
        lexemes,
        [
            "hoge",
            "fuga",
            "100",
            "3.14",
            "xx11__$$",
            "15",
            "abc",
            "abc0",
            ".5",
            ".5",
            "12.",
            "1.2",
            ".3",
            ".",
            "white",
            "space",
            "hoge",
            "1",
            "2",
            "/",
            "slash",
            "operators",
            ":",
            "+",
            "-",
            "*",
            "/",
            "==",
            "!=",
            "<",
            ">",
            "!",
            "&&",
            "||",
            "|",
            ":",
            ";",
            ",",
            "(",
            ")",
            "{",
            "}",
            "[",
            "]",
        ]
    );
}
