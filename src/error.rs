use crate::pos::{Pos, Range};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, Pos),
    #[error("unterminated comment (started at {0})")]
    UnterminatedComment(Pos),
    #[error("brace `{0}` opened at {1}, but unclosed until `{2}` at {3}")]
    UnclosedBraceUntil(String, Range, String, Range),
    #[error("brace `{0}` opened at {1}, but unclosed until end of file")]
    UnclosedBraceUntilEndOfFile(String, Range),
    #[error("unexpected token `{0}` at {1}")]
    UnexpectedToken(String, Range),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}
