use crate::pos::{CharPos, Pos};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, CharPos),
    #[error("unexpected character `{0}` after `{1}` for scientific notation at {2}")]
    UnexpectedCharacterAfterE(char, char, CharPos),
    #[error("unexpected end of line after `{0}` for scientific notation at {1}")]
    UnexpectedEndOfLineAfterE(char, CharPos),
    #[error("unterminated comment (started at {0})")]
    UnterminatedComment(CharPos),
    #[error("unterminated string literal (started at {0})")]
    UnterminatedStringLiteral(CharPos),
    #[error("brace `{0}` opened at {1}, but unclosed until `{2}` at {3}")]
    UnclosedBraceUntil(String, Pos, String, Pos),
    #[error("brace `{0}` opened at {1}, but unclosed until end of file")]
    UnclosedBraceUntilEndOfFile(String, Pos),
    #[error("unexpected token `{0}` at {1}")]
    UnexpectedToken(String, Pos),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
    #[error("cannot parse `{0}` at {1}: {2}")]
    FloatParseError(String, Pos, <f64 as std::str::FromStr>::Err),
    #[error("empty expression at {0}")]
    EmptyExpression(Pos),
}
