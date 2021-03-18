pub struct Char {
    value: char,
    line: usize,
    pos: usize,
}

impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Binary for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}

impl std::fmt::Debug for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`{0}` ({0:b})", self)
    }
}

pub fn read(path: &std::path::PathBuf) -> Result<Vec<Char>, Box<dyn std::error::Error>> {
    let mut ret = Vec::new();
    let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
    for line in 0.. {
        let mut buf = String::new();
        if std::io::BufRead::read_line(&mut reader, &mut buf)? == 0 {
            break;
        }
        for (pos, c) in buf.chars().enumerate() {
            ret.push(Char { value: c, line: line + 1, pos: pos + 1 });
        }
    }
    return Ok(ret);
}

pub enum Token<'char> {
    Identifier(&'char [Char]),
    Literal(&'char [Char]),
    Operator(Operator, &'char Char),
}

impl<'char> std::fmt::Display for Token<'char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Token::Identifier(s) | Token::Literal(s) => {
                for c in s {
                    write!(f, "{}", c.value)?
                }
                Ok(())
            }
            Token::Operator(_, c) => write!(f, "{}", c),
        }
    }
}

impl<'char> std::fmt::Binary for Token<'char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Token::Identifier(s) | Token::Literal(s) => write!(f, "({:b} - {:b})", s.first().unwrap(), s.last().unwrap()),
            Token::Operator(_, c) => write!(f, "{:b}", c),
        }
    }
}

impl<'char> std::fmt::Debug for Token<'char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`{0}` ({0:b})", self)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    ParenOpen,
    ParenClose,
    Semicolon,
    Bar,
    Colon,
    Equal,
    Comma,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
}

#[derive(thiserror::Error, Debug)]
pub enum LexerError<'char> {
    #[error("unexpected character `{0}` at {0:b}")]
    UnexpectedCharacter(&'char Char),
}

pub fn lexer(source: &[Char]) -> Result<Vec<Token>, LexerError> {
    let mut ret = Vec::new();
    enum State<'char> {
        Default,
        Alphabetic(usize),
        Digit(usize),
        Operator(Operator, &'char Char),
    }
    let mut prev = State::Default;
    for (next_index, c) in source.iter().enumerate() {
        let next = match c.value {
            '0'..='9' | '.' => State::Digit(next_index),
            'a'..='z' | 'A'..='Z' | '_' => State::Alphabetic(next_index),
            '+' => State::Operator(Operator::Plus, c),
            '-' => State::Operator(Operator::Minus, c),
            '*' => State::Operator(Operator::Asterisk, c),
            '/' => State::Operator(Operator::Slash, c),
            '(' => State::Operator(Operator::ParenOpen, c),
            ')' => State::Operator(Operator::ParenClose, c),
            ';' => State::Operator(Operator::Semicolon, c),
            '|' => State::Operator(Operator::Bar, c),
            ':' => State::Operator(Operator::Colon, c),
            '=' => State::Operator(Operator::Equal, c),
            ',' => State::Operator(Operator::Comma, c),
            '{' => State::Operator(Operator::BraceOpen, c),
            '}' => State::Operator(Operator::BraceClose, c),
            '[' => State::Operator(Operator::BracketOpen, c),
            ']' => State::Operator(Operator::BracketClose, c),
            '\t' | '\n' | '\x0C' | '\r' | ' ' => State::Default,
            _ => return Err(LexerError::UnexpectedCharacter(c)),
        };
        match prev {
            State::Alphabetic(prev_index) => match next {
                State::Alphabetic(_) | State::Digit(_) => continue,
                _ => ret.push(Token::Identifier(&source[prev_index..next_index])),
            },
            State::Digit(prev_index) => match next {
                State::Digit(_) => continue,
                _ => ret.push(Token::Literal(&source[prev_index..next_index])),
            },
            State::Operator(ty, c) => ret.push(Token::Operator(ty, c)),
            _ => {}
        }
        prev = next;
    }
    match prev {
        State::Alphabetic(prev_index) => ret.push(Token::Identifier(&source[prev_index..])),
        State::Digit(prev_index) => ret.push(Token::Literal(&source[prev_index..])),
        State::Operator(operator, c) => ret.push(Token::Operator(operator, c)),
        _ => {}
    }
    Ok(ret)
}

#[derive(Debug)]
pub enum Expression<'char> {
    Identifier(&'char [Char]),
    Literal(&'char [Char]),
    Prefix(Operator, &'char Char, Box<Expression<'char>>),
    Infix(Operator, &'char Char, Box<Expression<'char>>, Box<Expression<'char>>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError<'char, 'token> {
    #[error("parenthesis opened at `{0}` at {0:b}, not closed until `{1}` at {1:b}")]
    ParenthesisDoesNotMatch(&'char Char, &'char Char),
    #[error("no closing parenthesis to match `{0}` at {0:b}")]
    NoClosingParenthesis(&'char Char),
    #[error("unexpected operator `{0}` at {0:b}")]
    UnexpectedOperator(&'char Char),
    #[error("unexpected identifier `{0}` at {0:b}")]
    UnexpectedIdentifier(&'token Token<'char>),
    #[error("unexpected literal `{0}` at {0:b}")]
    UnexpectedLiteral(&'token Token<'char>),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}

fn parse_single_term<'token, 'char>(iter: &mut std::slice::Iter<'token, Token<'char>>) -> Result<Expression<'char>, ParseError<'char, 'token>> {
    match iter.next() {
        Some(&Token::Identifier(identifier)) => Ok(Expression::Identifier(identifier)),
        Some(&Token::Literal(literal)) => Ok(Expression::Literal(literal)),
        Some(&Token::Operator(operator, c)) => match operator {
            Operator::ParenOpen => match parse_expression(iter)? {
                (Some((Operator::ParenClose, _)), expression) => Ok(expression),
                (Some((_, end)), _) => Err(ParseError::ParenthesisDoesNotMatch(c, end)),
                (None, _) => Err(ParseError::NoClosingParenthesis(c)),
            },
            Operator::Plus | Operator::Minus | Operator::Asterisk | Operator::Slash => Ok(Expression::Prefix(operator, c, Box::new(parse_single_term(iter)?))),
            _ => Err(ParseError::UnexpectedOperator(c)),
        },
        None => Err(ParseError::UnexpectedEndOfFile),
    }
}
pub fn parse_expression<'token, 'char>(iter: &mut std::slice::Iter<'token, Token<'char>>) -> Result<(Option<(Operator, &'char Char)>, Expression<'char>), ParseError<'char, 'token>> {
    let mut ret = Vec::new();
    let mut operators = Vec::new();
    fn precedence(operator: Operator) -> i32 {
        match operator {
            Operator::Plus | Operator::Minus => 1,
            Operator::Asterisk | Operator::Slash => 2,
            _ => panic!(),
        }
    }
    fn operate<'char>(vec: &mut Vec<Expression<'char>>, operator: Operator, c: &'char Char) {
        let expression2 = vec.pop().unwrap();
        let expression1 = vec.pop().unwrap();
        vec.push(Expression::Infix(operator, c, Box::new(expression1), Box::new(expression2)));
    }
    let last = loop {
        ret.push(parse_single_term(iter)?);
        match iter.next() {
            Some(&Token::Operator(operator, c)) => match operator {
                Operator::Plus | Operator::Minus | Operator::Asterisk | Operator::Slash => {
                    while let Some(&(last, lc)) = operators.last() {
                        if precedence(last) >= precedence(operator) {
                            operate(&mut ret, last, lc);
                            operators.pop();
                        } else {
                            break;
                        }
                    }
                    operators.push((operator, c));
                }
                _ => break Some((operator, c)),
            },
            Some(token @ Token::Identifier(_)) => return Err(ParseError::UnexpectedIdentifier(token)),
            Some(token @ Token::Literal(_)) => return Err(ParseError::UnexpectedLiteral(token)),
            None => break None,
        }
    };
    while let Some((last, lc)) = operators.pop() {
        operate(&mut ret, last, lc);
    }
    Ok((last, ret.pop().unwrap()))
}

#[derive(Debug)]
pub enum Score<'char> {
    Note(Option<&'char [Char]>, Option<&'char [Char]>),
    Map(Box<Score<'char>>, Vec<(&'char [Char], Expression<'char>)>, Vec<(&'char [Char], Expression<'char>)>),
    Row(Vec<Score<'char>>),
    Column(Vec<Score<'char>>),
}

pub fn parse<'token, 'char>(iter: &mut std::slice::Iter<'token, Token<'char>>) {
    
}
