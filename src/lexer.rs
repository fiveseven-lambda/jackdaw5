use crate::token::{Bracket, Operator, Token, TokenName};
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
    fn read_line(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.prompt {
            // 対話環境ではプロンプトを出す
            // ファイルから読むときは出さない
            use std::io::Write;
            print!("> ");
            std::io::stdout().flush()?;
        }
        let mut s = String::new();
        if self.reader.read_line(&mut s)? == 0 {
            // end of file
            return match self.comment.pop() {
                Some(pos) => Err(Error::UnterminatedComment(pos).into()), // コメントのまま終了した
                None => Ok(false),
            };
        }

        self.line += 1;

        enum State {
            Initial,
            Identifier,
            Number { decimal: bool },
            Operator(Operator),
        }

        let mut prev = State::Initial;
        let mut prev_index = 0;
        let mut prev_pos = 0;

        let mut iter = s.char_indices().enumerate().peekable();

        while let Some((pos, (index, c))) = iter.next() {
            if self.comment.len() > 0 {
                if c == '*' {
                    if let Some((_, (_, '/'))) = iter.peek() {
                        iter.next(); // peek した '/' を読む
                        self.comment.pop(); // コメントの終了
                    }
                } else if c == '/' {
                    match iter.peek() {
                        Some((_, (_, '*'))) => {
                            iter.next(); // peek した '*' を読む
                            self.comment.push(self.pos(pos)); // コメントの開始（ネスト）
                        }
                        Some((_, (_, '/'))) => {
                            return Ok(true); // ラインコメント
                        }
                        _ => {}
                    }
                }
                continue;
            }
            let next = match c {
                'A'..='Z' | 'a'..='z' | '_' | '$' => State::Identifier,
                '0'..='9' => State::Number { decimal: false },
                '.' => State::Number { decimal: true },
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
                    '(' => Operator::Open(Bracket::Round),
                    ')' => Operator::Close(Bracket::Round),
                    '{' => Operator::Open(Bracket::Curly),
                    '}' => Operator::Close(Bracket::Curly),
                    '[' => Operator::Open(Bracket::Square),
                    ']' => Operator::Close(Bracket::Square),
                    _ => return Err(Error::UnexpectedCharacter(c, self.pos(pos)).into()),
                }),
            };
            prev = match (prev, next) {
                (State::Identifier, State::Identifier | State::Number { decimal: false }) => State::Identifier,
                (State::Number { decimal: prev_d }, State::Number { decimal: next_d }) if !(prev_d && next_d) => {
                    State::Number { decimal: prev_d || next_d }
                }
                (State::Operator(Operator::Slash), State::Operator(Operator::Slash)) => return Ok(true), // ラインコメント
                (State::Operator(Operator::Slash), State::Operator(Operator::Asterisk)) => {
                    self.comment.push(self.pos(prev_pos)); // ブロックコメントの開始
                    prev = State::Initial;
                    continue;
                }
                // 2 文字でできた演算子
                (State::Operator(Operator::Exclamation), State::Operator(Operator::Equal)) => State::Operator(Operator::ExclamationEqual),
                (State::Operator(Operator::Equal), State::Operator(Operator::Equal)) => State::Operator(Operator::DoubleEqual),
                (State::Operator(Operator::Ampersand), State::Operator(Operator::Ampersand)) => State::Operator(Operator::DoubleAmpersand),
                (State::Operator(Operator::Bar), State::Operator(Operator::Bar)) => State::Operator(Operator::DoubleBar),
                (prev, next) => {
                    'push: loop {
                        // labeled-block が安定化されたら書き直す
                        break self.queue.push_back(Token {
                            name: match prev {
                                State::Initial => break 'push,
                                State::Identifier => TokenName::Identifier,
                                State::Operator(operator) => TokenName::Operator(operator),
                                State::Number { .. } => TokenName::Number,
                            },
                            lexeme: s[prev_index..index].to_string(),
                            pos: self.pos(prev_pos),
                        });
                    }
                    prev_index = index;
                    prev_pos = pos;
                    next
                }
            };
        }

        Ok(true)
    }
    pub fn next(&mut self) -> Result<Option<Token>, Box<dyn std::error::Error>> {
        let ret = self.queue.pop_front();
        if ret.is_none() && self.read_line()? {
            return self.next();
        }
        Ok(ret)
    }
}
