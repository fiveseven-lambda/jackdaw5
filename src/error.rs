use crate::pos::Pos;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, Pos),
    #[error("unterminated comment (started at {0})")]
    UnterminatedComment(Pos),
    #[error("parenthesis opened at {0}, but unclosed until `{1}` at {2}")]
    UnclosedParenthesisUntil(Pos, String, Pos),
    #[error("parenthesis opened at {0}, but unclosed until end of file")]
    UnclosedParenthesisUntilEndOfFile(Pos),
    #[error("unexpected token `{0}` at {1}")]
    UnexpectedToken(String, Pos),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
}
