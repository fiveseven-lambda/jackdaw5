use crate::token::{Token, TokenName};
use std::collections::VecDeque;

use crate::error::Error;
use crate::pos::{CharPos, Pos};

pub struct Lexer<BufRead> {
    reader: BufRead,
    prompt: bool,
    queue: VecDeque<Token>,
    line: usize,           // 今何行目か
    comment: Vec<CharPos>, // コメントの開始点
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
    fn char_pos(&self, column: usize) -> CharPos {
        CharPos::new(self.line, column)
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
            return match self.comment.pop() {
                Some(start) => Err(Error::UnterminatedComment(start).into()), // コメントのまま終了した
                None => Ok(false),
            };
        }

        self.line += 1;

        let mut prev: Option<TokenName> = None;
        let mut start_index = 0;
        let mut start_column = 0;

        let mut string: bool = false;

        let mut iter = s.char_indices().enumerate().peekable();

        while let Some((column, (index, c))) = iter.next() {
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
                            self.comment.push(self.char_pos(column)); // コメントの開始（ネスト）
                        }
                        Some((_, (_, '/'))) => {
                            return Ok(true); // ラインコメント
                        }
                        _ => {}
                    }
                }
                continue;
            }
            if string {
                if c == '"' {
                    string = false;
                } else if c == '\\' {
                    iter.next();
                }
                continue;
            }
            let next = match c {
                'A'..='Z' | 'a'..='z' | '_' => Some(TokenName::Identifier { dollar: false }),
                '$' => Some(TokenName::Identifier { dollar: true }),
                '0'..='9' => Some(TokenName::Number),
                '"' => {
                    string = true;
                    Some(TokenName::String)
                }
                c if c.is_ascii_whitespace() => None,
                '+' => Some(TokenName::Plus),
                '-' => Some(TokenName::Minus),
                '*' => Some(TokenName::Asterisk),
                '^' => Some(TokenName::Circumflex),
                '/' => Some(TokenName::Slash),
                '=' => Some(TokenName::Equal),
                '!' => Some(TokenName::Exclamation),
                '<' => Some(TokenName::Less),
                '>' => Some(TokenName::Greater),
                '&' => Some(TokenName::Ampersand),
                '|' => Some(TokenName::Bar),
                ':' => Some(TokenName::Colon),
                ';' => Some(TokenName::Semicolon),
                ',' => Some(TokenName::Comma),
                '.' => Some(TokenName::Dot),
                '(' => Some(TokenName::OpeningParen),
                ')' => Some(TokenName::ClosingParen),
                '{' => Some(TokenName::OpeningBrace),
                '}' => Some(TokenName::ClosingBrace),
                '[' => Some(TokenName::OpeningBracket),
                ']' => Some(TokenName::ClosingBracket),
                _ => return Err(Error::UnexpectedCharacter(c, self.char_pos(column)).into()),
            };
            prev = match (prev, next) {
                (Some(TokenName::Identifier { dollar }), Some(TokenName::Identifier { .. } | TokenName::Number)) => {
                    Some(TokenName::Identifier { dollar })
                }
                (Some(TokenName::Number), Some(TokenName::Number) | Some(TokenName::Dot)) => Some(TokenName::Number),
                (Some(TokenName::Dot), Some(TokenName::Number)) => Some(TokenName::Number),
                (Some(TokenName::Number), _) if c == 'e' || c == 'E' => match iter.next() {
                    Some((_, (_, '+' | '-' | '0'..='9'))) => Some(TokenName::Number),
                    Some((_, (_, other))) => return Err(Error::UnexpectedCharacterAfterE(other, c, self.char_pos(column)).into()),
                    None => return Err(Error::UnexpectedEndOfLineAfterE(c, self.char_pos(column)).into()),
                },
                (Some(TokenName::Slash), Some(TokenName::Slash)) => return Ok(true),
                (Some(TokenName::Slash), Some(TokenName::Asterisk)) => {
                    self.comment.push(self.char_pos(start_column));
                    prev = None;
                    continue;
                }
                (Some(TokenName::Exclamation), Some(TokenName::Equal)) => Some(TokenName::ExclamationEqual),
                (Some(TokenName::Equal), Some(TokenName::Equal)) => Some(TokenName::DoubleEqual),
                (Some(TokenName::Ampersand), Some(TokenName::Ampersand)) => Some(TokenName::DoubleAmpersand),
                (Some(TokenName::Bar), Some(TokenName::Bar)) => Some(TokenName::DoubleBar),
                (Some(TokenName::Less), Some(TokenName::Less)) => Some(TokenName::DoubleLess),
                (Some(TokenName::Greater), Some(TokenName::Greater)) => Some(TokenName::DoubleGreater),
                (prev, next) => {
                    // トークンの終了
                    if let Some(name) = prev {
                        self.queue.push_back(Token {
                            name,
                            lexeme: s[start_index..index].to_string(),
                            pos: Pos::new(self.char_pos(start_column), self.char_pos(column)),
                        });
                    }
                    // 新しいトークンの開始
                    start_index = index;
                    start_column = column;
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
