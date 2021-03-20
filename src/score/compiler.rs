use crate::pos::Pos;

#[derive(thiserror::Error, Debug)]
pub enum CompileError<'s, 'p> {
    #[error("IllegalLiteral {0} at {1} ({2})")]
    IllegalLiteral(&'s str, &'p Pos, std::num::ParseFloatError),
}
