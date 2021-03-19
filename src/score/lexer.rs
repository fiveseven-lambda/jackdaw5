use super::token::{Operator, Token, TokenPos};
use crate::pos::{CharPos, Pos};

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, Pos),
}

pub fn lexer(source: &str) -> Result<Vec<TokenPos>, LexerError> {
    enum CharType {
        Digit(usize),
        Alphabetic(usize),
        Operator(Operator),
    }

    let mut prev: Option<(CharType, Pos)> = None;

    let mut ret = Vec::new();

    for (next_index, next_char, next_pos) in source.char_indices().pos() {
        let next = match next_char {
            '0'..='9' | '.' => Some((CharType::Digit(next_index), next_pos)),
            'A'..='Z' | 'a'..='z' | '_' => Some((CharType::Alphabetic(next_index), next_pos)),
            c if c.is_ascii_whitespace() => None,
            '+' => Some((CharType::Operator(Operator::Plus), next_pos)),
            '-' => Some((CharType::Operator(Operator::Minus), next_pos)),
            '*' => Some((CharType::Operator(Operator::Asterisk), next_pos)),
            '/' => Some((CharType::Operator(Operator::Slash), next_pos)),
            '(' => Some((CharType::Operator(Operator::ParenOpen), next_pos)),
            ')' => Some((CharType::Operator(Operator::ParenClose), next_pos)),
            ';' => Some((CharType::Operator(Operator::Semicolon), next_pos)),
            '|' => Some((CharType::Operator(Operator::Bar), next_pos)),
            ':' => Some((CharType::Operator(Operator::Colon), next_pos)),
            '=' => Some((CharType::Operator(Operator::Equal), next_pos)),
            ',' => Some((CharType::Operator(Operator::Comma), next_pos)),
            '{' => Some((CharType::Operator(Operator::BraceOpen), next_pos)),
            '}' => Some((CharType::Operator(Operator::BraceClose), next_pos)),
            '[' => Some((CharType::Operator(Operator::BracketOpen), next_pos)),
            ']' => Some((CharType::Operator(Operator::BracketClose), next_pos)),
            c => return Err(LexerError::UnexpectedCharacter(c, next_pos)),
        };
        let tuple = (prev, next);
        match tuple {
            (Some((CharType::Alphabetic(_), _)), Some((CharType::Alphabetic(_), _))) | (Some((CharType::Alphabetic(_), _)), Some((CharType::Digit(_), _))) | (Some((CharType::Digit(_), _)), Some((CharType::Digit(_), _))) => {
                prev = tuple.0;
                continue;
            }
            (Some((CharType::Alphabetic(prev_index), prev_pos)), _) => ret.push(TokenPos { token: Token::Identifier(&source[prev_index..next_index]), pos: prev_pos }),
            (Some((CharType::Digit(prev_index), prev_pos)), _) => ret.push(TokenPos { token: Token::Literal(&source[prev_index..next_index]), pos: prev_pos }),
            (Some((CharType::Operator(operator), prev_pos)), _) => ret.push(TokenPos { token: Token::Operator(operator), pos: prev_pos }),
            (None, _) => {}
        }
        prev = tuple.1;
    }
    Ok(ret)
}
