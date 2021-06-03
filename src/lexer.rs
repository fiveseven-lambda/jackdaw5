use crate::token::{Bracket, Operator, Token, TokenName};
use std::collections::VecDeque;

use crate::error::Error;
use crate::pos::{End, Pos};

pub struct Lexer<BufRead> {
    reader: BufRead,
    prompt: bool,
    queue: VecDeque<Token>,
    line: usize,       // 今何行目か
    comment: Vec<End>, // コメントの開始点
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
    fn end(&self, column: usize) -> End {
        End::new(self.line, column + 1)
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
                Some(start) => Err(Error::UnterminatedComment(start).into()), // コメントのまま終了した
                None => Ok(false),
            };
        }

        self.line += 1;

        enum State {
            Initial,
            Identifier { dollar: bool },
            Number { decimal: bool },
            Operator(Operator),
        }

        let mut prev = State::Initial;
        let mut prev_index = 0;
        let mut prev_column = 0;
        let mut last_column = 0;

        // はじめ prev は State::Initial
        // トークンが始まると prev は State::Initial 以外になり，
        // prev_index にはそのときのバイト位置，
        // prev_column には column （何文字目か）が入る
        // トークンが終わるまで prev_index, prev_column は更新されないが
        // last_column には column が代入され続ける
        // トークンが終わり，次の文字が読まれた段階で
        // バイト位置 prev_index..index がトークン
        // prev_column が最初の文字の位置， last_column が最後の文字の位置

        // prev が State::Initial のうちは prev_index, prev_column, last_column の値は意味をなさないので
        // prev の代わりに Option<(State, usize, usize, usize)> を用いて
        // State::Initial の代わりに None とする方が良いかもしれない

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
                            self.comment.push(self.end(column)); // コメントの開始（ネスト）
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
                // もし State::Initial のときに c が現れたら
                // State は何になるか？
                'A'..='Z' | 'a'..='z' | '_' => State::Identifier { dollar: false },
                '$' => State::Identifier { dollar: true },
                '0'..='9' => State::Number { decimal: false },
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
                    '.' => Operator::Dot,
                    '(' => Operator::Open(Bracket::Round),
                    ')' => Operator::Close(Bracket::Round),
                    '{' => Operator::Open(Bracket::Curly),
                    '}' => Operator::Close(Bracket::Curly),
                    '[' => Operator::Open(Bracket::Square),
                    ']' => Operator::Close(Bracket::Square),
                    _ => return Err(Error::UnexpectedCharacter(c, self.end(column)).into()),
                }),
            };
            prev = match (prev, next) {
                (State::Identifier { dollar }, State::Identifier { .. } | State::Number { .. }) => State::Identifier { dollar },
                (State::Number { decimal }, State::Number { .. }) => State::Number { decimal }, // 数値リテラルに続く数字
                (State::Number { decimal: false }, State::Operator(Operator::Dot)) => State::Number { decimal: true }, // 数値リテラル中に現れる小数点
                (State::Operator(Operator::Dot), State::Number { .. }) => State::Number { decimal: true }, // 小数点から始まる数値リテラル
                (State::Operator(Operator::Slash), State::Operator(Operator::Slash)) => return Ok(true), // ラインコメント
                (State::Operator(Operator::Slash), State::Operator(Operator::Asterisk)) => {
                    self.comment.push(self.end(prev_column)); // ブロックコメントの開始
                    prev = State::Initial;
                    continue;
                }
                // 2 文字でできた演算子
                (State::Operator(Operator::Exclamation), State::Operator(Operator::Equal)) => State::Operator(Operator::ExclamationEqual),
                (State::Operator(Operator::Equal), State::Operator(Operator::Equal)) => State::Operator(Operator::DoubleEqual),
                (State::Operator(Operator::Ampersand), State::Operator(Operator::Ampersand)) => State::Operator(Operator::DoubleAmpersand),
                (State::Operator(Operator::Bar), State::Operator(Operator::Bar)) => State::Operator(Operator::DoubleBar),
                (State::Operator(Operator::Less), State::Operator(Operator::Less)) => State::Operator(Operator::DoubleLess),
                (State::Operator(Operator::Greater), State::Operator(Operator::Greater)) => State::Operator(Operator::DoubleGreater),
                (prev, next) => {
                    'push: loop {
                        // labeled-block が安定化されたら書き直す
                        break self.queue.push_back(Token {
                            name: match prev {
                                State::Initial => break 'push,
                                State::Identifier { dollar } => TokenName::Identifier { dollar },
                                State::Operator(operator) => TokenName::Operator(operator),
                                State::Number { .. } => TokenName::Number,
                            },
                            lexeme: s[prev_index..index].to_string(),
                            pos: Pos::new(self.end(prev_column), self.end(last_column)),
                        });
                    }
                    prev_index = index;
                    prev_column = column;
                    next
                }
            };
            last_column = column;
        }
        // 最後に何か残っていたとき
        self.queue.push_back(Token {
            name: match prev {
                State::Initial => return Ok(true),
                State::Identifier { dollar } => TokenName::Identifier { dollar },
                State::Operator(operator) => TokenName::Operator(operator),
                State::Number { .. } => TokenName::Number,
            },
            lexeme: s[prev_index..].to_string(),
            pos: Pos::new(self.end(prev_column), self.end(last_column)),
        });

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
