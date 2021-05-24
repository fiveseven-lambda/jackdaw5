use crate::token::{Bracket, Operator, Token, TokenName};
use std::collections::VecDeque;

use crate::error::Error;
use crate::pos::{Pos, Range};

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

        let mut last_index = 0;

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
                            range: Range::new(self.pos(prev_pos), self.pos(last_index)),
                        });
                    }
                    prev_index = index;
                    prev_pos = pos;
                    next
                }
            };
            last_index = index;
        }
        // 最後に何か残っていたとき
        self.queue.push_back(Token {
            name: match prev {
                State::Initial => return Ok(true),
                State::Identifier => TokenName::Identifier,
                State::Operator(operator) => TokenName::Operator(operator),
                State::Number { .. } => TokenName::Number,
            },
            lexeme: s[prev_index..].to_string(),
            range: Range::new(self.pos(prev_pos), self.pos(last_index)),
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

#[test]
fn main() {
    let input: &[u8] = b"
    hoge fuga
    100  3.14
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
    ( ) { } [ ]";

    let mut lexer = Lexer::new(input, false);

    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Identifier));
        assert_eq!(token.lexeme, "hoge");
        assert_eq!(token.range.into_inner(), (2, 5)..=(2, 8));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Identifier));
        assert_eq!(token.lexeme, "fuga");
        assert_eq!(token.range.into_inner(), (2, 10)..=(2, 13));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Number));
        assert_eq!(token.lexeme, "100");
        assert_eq!(token.range.into_inner(), (3, 5)..=(3, 7));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Number));
        assert_eq!(token.lexeme, "3.14");
        assert_eq!(token.range.into_inner(), (3, 10)..=(3, 13));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Identifier));
        assert_eq!(token.lexeme, "xx11__$$");
        assert_eq!(token.range.into_inner(), (4, 5)..=(4, 12));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Number));
        assert_eq!(token.lexeme, "15");
        assert_eq!(token.range.into_inner(), (5, 5)..=(5, 6));
    }
    {
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.name, TokenName::Identifier));
        assert_eq!(token.lexeme, "abc");
        assert_eq!(token.range.into_inner(), (5, 7)..=(5, 9));
    }
    // assert!(lexer.next().unwrap().is_none());
}
