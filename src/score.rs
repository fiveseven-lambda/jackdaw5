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

pub enum Token<'c> {
    Identifier(&'c [Char]),
    Literal(&'c [Char]),
    Operator(Operator, &'c Char),
}

impl<'c> std::fmt::Display for Token<'c> {
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

impl<'c> std::fmt::Binary for Token<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Token::Identifier(s) | Token::Literal(s) => write!(f, "({:b} - {:b})", s.first().unwrap(), s.last().unwrap()),
            Token::Operator(_, c) => write!(f, "{:b}", c),
        }
    }
}

impl<'c> std::fmt::Debug for Token<'c> {
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
pub enum LexerError<'c> {
    #[error("unexpected character `{0}` at {0:b}")]
    UnexpectedCharacter(&'c Char),
}

pub fn lexer(source: &[Char]) -> Result<Vec<Token>, LexerError> {
    let mut ret = Vec::new();
    enum State<'c> {
        Default,
        Alphabetic(usize),
        Digit(usize),
        Operator(Operator, &'c Char),
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
pub enum Expression<'c> {
    Identifier(&'c [Char]),
    Literal(&'c [Char]),
    Prefix(Operator, &'c Char, Box<Expression<'c>>),
    Infix(Operator, &'c Char, Box<Expression<'c>>, Box<Expression<'c>>),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError<'c, 't> {
    #[error("parenthesis opened at `{0}` at {0:b}, not closed until `{1}` at {1:b}")]
    ParenthesisDoesNotMatch(&'c Char, &'c Char),
    #[error("no closing parenthesis to match `{0}` at {0:b}")]
    NoClosingParenthesis(&'c Char),
    #[error("unexpected operator `{0}` at {0:b}")]
    UnexpectedOperator(&'c Char),
    #[error("unexpected token `{0}` at {0:b}")]
    UnexpectedToken(&'t Token<'c>),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}

fn parse_single_term<'t, 'c>(iter: &mut std::slice::Iter<'t, Token<'c>>) -> Result<Expression<'c>, ParseError<'c, 't>> {
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

fn parse_expression<'t, 'c>(iter: &mut std::slice::Iter<'t, Token<'c>>) -> Result<(Option<(Operator, &'c Char)>, Expression<'c>), ParseError<'c, 't>> {
    let mut ret = Vec::new();
    let mut operators = Vec::new();
    fn precedence(operator: Operator) -> i32 {
        match operator {
            Operator::Plus | Operator::Minus => 1,
            Operator::Asterisk | Operator::Slash => 2,
            _ => panic!(),
        }
    }
    fn operate<'c>(vec: &mut Vec<Expression<'c>>, operator: Operator, c: &'c Char) {
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
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => break None,
        }
    };
    while let Some((last, lc)) = operators.pop() {
        operate(&mut ret, last, lc);
    }
    Ok((last, ret.pop().unwrap()))
}

#[derive(Debug)]
pub enum Notes<'c> {
    Note(Option<&'c [Char]>, Option<&'c [Char]>),
    Identifier(&'c [Char]),
    Row(Vec<Score<'c>>),
    Column(Vec<Score<'c>>),
}

#[derive(Debug)]
pub struct Map<'c> {
    condition: Vec<(Expression<'c>, Expression<'c>)>,
    assignment: Vec<(&'c [Char], Expression<'c>)>,
}

#[derive(Debug)]
pub struct Score<'c> {
    notes: Notes<'c>,
    map: Vec<Map<'c>>,
}

pub fn parse_map<'c, 't>(iter: &mut std::slice::Iter<'t, Token<'c>>) -> Result<(Option<(Operator, &'c Char)>, Map<'c>), ParseError<'c, 't>> {
    let mut condition = Vec::new();
    loop {
        let (equal, left) = match parse_expression(iter) {
            Ok(result) => result,
            Err(ParseError::UnexpectedOperator(c)) if c.value == ':' => break,
            Err(err) => return Err(err),
        };
        match equal {
            Some((Operator::Equal, _)) => {
                let (end, right) = parse_expression(iter)?;
                condition.push((left, right));
                match end {
                    Some((Operator::Comma, _)) => {}
                    Some((Operator::Colon, _)) => break,
                    Some((_, c)) => return Err(ParseError::UnexpectedOperator(c)),
                    None => return Err(ParseError::UnexpectedEndOfFile),
                }
            }
            Some((_, c)) => return Err(ParseError::UnexpectedOperator(c)),
            None => return Err(ParseError::UnexpectedEndOfFile),
        }
    }
    let mut assignment = Vec::new();
    let end = loop {
        match iter.next() {
            Some(&Token::Identifier(identifier)) => match iter.next() {
                Some(Token::Operator(Operator::Equal, _)) => {
                    let (end, expression) = parse_expression(iter)?;
                    assignment.push((identifier, expression));
                    match end {
                        Some((Operator::Comma, _)) => {}
                        end => break end,
                    }
                }
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEndOfFile),
            },
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEndOfFile),
        }
    };
    Ok((end, Map { condition: condition, assignment: assignment }))
}

pub fn parse_score<'c, 't>(iter: &mut std::slice::Iter<'t, Token<'c>>) -> Result<(Option<(Operator, &'c Char)>, Vec<Score<'c>>), ParseError<'c, 't>> {
    let mut ret = Vec::new();
    while let Some(token) = iter.next() {
        let (end, notes) = match token {
            Token::Operator(Operator::BraceOpen, _) => match parse_score(iter)? {
                (Some((Operator::BraceClose, _)), score) => (iter.next(), Notes::Row(score)),
                (Some((_, c)), _) => return Err(ParseError::UnexpectedOperator(c)),
                (None, _) => return Err(ParseError::UnexpectedEndOfFile),
            },
            Token::Operator(Operator::BracketOpen, _) => match parse_score(iter)? {
                (Some((Operator::BracketClose, _)), score) => (iter.next(), Notes::Column(score)),
                (Some((_, c)), _) => return Err(ParseError::UnexpectedOperator(c)),
                (None, _) => return Err(ParseError::UnexpectedEndOfFile),
            },
            Token::Literal(_) | Token::Operator(Operator::Slash, _) => {
                let (first, mut slash) = match token {
                    &Token::Literal(literal) => (Some(literal), false),
                    Token::Operator(Operator::Slash, _) => (None, true),
                    _ => unreachable!(),
                };
                let mut second = None;
                loop {
                    match iter.next() {
                        Some(&Token::Literal(literal)) => {
                            if slash && second.is_none() {
                                second = Some(literal);
                            } else {
                                panic!();
                            }
                        }
                        Some(Token::Operator(Operator::Slash, _)) => {
                            if !slash {
                                slash = true;
                            } else {
                                panic!();
                            }
                        }
                        other => break (other, Notes::Note(first, second)),
                    }
                }
            }
            Token::Identifier(identifier) => (iter.next(), Notes::Identifier(identifier)),
            Token::Operator(_, c) => return Err(ParseError::UnexpectedOperator(c)),
        };
        ret.push(Score { notes: notes, map: Vec::new() });
        match end {
            Some(Token::Operator(Operator::Semicolon, _)) => continue,
            Some(Token::Operator(Operator::Bar, _)) => {}
            Some(&Token::Operator(operator, c)) => return Ok((Some((operator, c)), ret)),
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Ok((None, ret)),
        }
        loop {
            let (end, map) = parse_map(iter)?;
            ret.last_mut().unwrap().map.push(map);
            match end {
                Some((Operator::Bar, _)) => {}
                Some((Operator::Semicolon, _)) => break,
                other => return Ok((other, ret)),
            }
        }
    }
    Ok((None, ret))
}

pub fn parse<'c, 't>(iter: &mut std::slice::Iter<'t, Token<'c>>) -> Result<Notes<'c>, ParseError<'c, 't>> {
    let (end, score) = parse_score(iter)?;
    match end {
        Some((_, c)) => Err(ParseError::UnexpectedOperator(c)),
        None => Ok(Notes::Row(score))
    }
}
