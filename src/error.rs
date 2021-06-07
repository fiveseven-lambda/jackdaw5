use crate::pos::{End, Pos};
use crate::value::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, End),
    #[error("unterminated comment (started at {0})")]
    UnterminatedComment(End),
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
    #[error("type mismatch (operand is {0:?}) at {1}")]
    TypeMismatchUnary(Value, Pos),
    #[error("type mismatch (operands are {0:?} and {1:?}) at {2}")]
    TypeMismatchBinary(Value, Value, Pos),
    #[error("not a function (at {0})")]
    NotAFunction(Pos),
    #[error("undefined variable `{0}` at {1}")]
    UndefinedVariable(String, Pos),
}
