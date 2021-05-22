use crate::pos::Pos;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at `{1}`")]
    UnexpectedCharacter(char, Pos),
    #[error("unterminated comment (started at `{0}`)")]
    UnterminatedComment(Pos),
}
