use crate::pos::{CharPos, Pos};
use crate::value::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at {1}")]
    UnexpectedCharacter(char, CharPos),
    #[error("unexpected character `{0}` after `{1}` for scientific notation at {2}")]
    UnexpectedCharacterAfterE(char, char, CharPos),
    #[error("unexpected end of line after `{0}` for scientific notation at {1}")]
    UnexpectedEndOfLineAfterE(char, CharPos),
    #[error("token `{0}` at end of line")]
    TokenAtEndOfLine(String),
    #[error("unterminated comment (started at {0})")]
    UnterminatedComment(CharPos),
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
    #[error("type mismatch: operator - (minus) expected real or Sound, but found {0:?} at {1}")]
    TypeMismatchMinus(Value, Pos),
    #[error("type mismatch: operator / (reciprocal) expected real or Sound, but found {0:?} at {1}")]
    TypeMismatchReciprocal(Value, Pos),
    #[error("type mismatch: operator ! (negation) expected bool, but found {0:?} at {1}")]
    TypeMismatchNot(Value, Pos),
    #[error("type mismatch: operator + (addition) expected real, Sound or string, but found {0:?} and {1:?} at {2}")]
    TypeMismatchAdd(Value, Value, Pos),
    #[error("type mismatch: operator - (subtraction) expected real or Sound, but found {0:?} and {1:?} at {2}")]
    TypeMismatchSub(Value, Value, Pos),
    #[error("type mismatch: operator * (multiplication) expected real or Sound, but found {0:?} and {1:?} at {2}")]
    TypeMismatchMul(Value, Value, Pos),
    #[error("type mismatch: operator / (division) expected real or Sound, but found {0:?} and {1:?} at {2}")]
    TypeMismatchDiv(Value, Value, Pos),
    #[error("type mismatch: operator ^ (power) expected real or Sound, but found {0:?} and {1:?} at {2}")]
    TypeMismatchPow(Value, Value, Pos),
    #[error("type mismatch: operator < (less) expected real, but found {0:?} and {1:?} at {2}")]
    TypeMismatchLess(Value, Value, Pos),
    #[error("type mismatch: operator > (greater) expected real, but found {0:?} and {1:?} at {2}")]
    TypeMismatchGreater(Value, Value, Pos),
    #[error("type mismatch: operator << (time shift) expected Sound and real, but found {0:?} and {1:?} at {2}")]
    TypeMismatchLeftShift(Value, Value, Pos),
    #[error("type mismatch: operator >> (time shift) expected Sound and real, but found {0:?} and {1:?} at {2}")]
    TypeMismatchRightShift(Value, Value, Pos),
    #[error("type mismatch: operator == (equal) expected real, string or bool, but found {0:?} and {1:?} at {2}")]
    TypeMismatchEqual(Value, Value, Pos),
    #[error("type mismatch: operator != (not equal) expected real, string or bool, but found {0:?} and {1:?} at {2}")]
    TypeMismatchNotEqual(Value, Value, Pos),
    #[error("type mismatch: operator && (and) expected bool, but found {0:?} at {1}")]
    TypeMismatchAnd1(Value, Pos),
    #[error("type mismatch: operator && (and) expected bool, but found {0:?} and {1:?} at {2}")]
    TypeMismatchAnd2(Value, Value, Pos),
    #[error("type mismatch: operator || (or) expected bool, but found {0:?} at {1}")]
    TypeMismatchOr1(Value, Pos),
    #[error("type mismatch: operator || (or) expected bool, but found {0:?} and {1:?} at {2}")]
    TypeMismatchOr2(Value, Value, Pos),
    #[error("not a function (at {0})")]
    NotAFunction(Pos),
    #[error("undefined variable `{0}` at {1}")]
    UndefinedVariable(String, Pos),
}
