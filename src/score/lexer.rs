use super::token::{Operator, Token, TokenPos};
use crate::pos::{CharPos, Pos};

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, Pos),
}

pub fn lexer(source: &str) -> Result<Vec<TokenPos>, LexerError> {
    enum CharType {
        Space,
        Digit(usize),
        Alphabetic(usize),
        Operator(Operator),
    }

    let mut prev: Option<(CharType, Pos)> = None;

    let mut ret = Vec::new();

    for (next_index, next_char, next_pos) in source.char_indices().pos() {
        let next = Some((match next_char {
            '0'..='9' | '.' => CharType::Digit(next_index),
            'A'..='Z' | 'a'..='z' | '_' => CharType::Alphabetic(next_index),
            c if c.is_ascii_whitespace() => CharType::Space,
            '+' => CharType::Operator(Operator::Plus),
            '-' => CharType::Operator(Operator::Minus),
            '*' => CharType::Operator(Operator::Asterisk),
            '/' => CharType::Operator(Operator::Slash),
            '(' => CharType::Operator(Operator::ParenOpen),
            ')' => CharType::Operator(Operator::ParenClose),
            ';' => CharType::Operator(Operator::Semicolon),
            '|' => CharType::Operator(Operator::Bar),
            ':' => CharType::Operator(Operator::Colon),
            '=' => CharType::Operator(Operator::Equal),
            ',' => CharType::Operator(Operator::Comma),
            '{' => CharType::Operator(Operator::BraceOpen),
            '}' => CharType::Operator(Operator::BraceClose),
            '[' => CharType::Operator(Operator::BracketOpen),
            ']' => CharType::Operator(Operator::BracketClose),
            c => return Err(LexerError::UnexpectedCharacter(c, next_pos)),
        }, next_pos));
        let tuple = (prev, next);
        prev = match tuple {
            | (Some((CharType::Alphabetic(_), _)), Some((CharType::Alphabetic(_), _)))
            | (Some((CharType::Alphabetic(_), _)), Some((CharType::Digit(_), _)))
            | (Some((CharType::Digit(_), _)), Some((CharType::Digit(_), _))) => {
                tuple.0
            }
            (Some((CharType::Alphabetic(prev_index), prev_pos)), next) => {
                ret.push(TokenPos { token: Token::Identifier(&source[prev_index..next_index]), pos: prev_pos });
                next
            }
            (Some((CharType::Digit(prev_index), prev_pos)), next) => {
                ret.push(TokenPos { token: Token::Literal(&source[prev_index..next_index]), pos: prev_pos });
                next
            }
            (Some((CharType::Operator(operator), prev_pos)), next) => {
                ret.push(TokenPos { token: Token::Operator(operator), pos: prev_pos });
                next
            }
            (_, next) => {
                next
            }
        };
    }
    Ok(ret)
}
