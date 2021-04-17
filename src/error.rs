#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at line {1}")]
    UnexpectedCharacter(char, usize),
    #[error("single ampersand `&` at line `{0}`; use `&&` instead")]
    SingleAmpersand(usize),
    #[error("unexpected end of file")]
    UnexpectedEndOfFile,
    #[error("unexpected operator `{0}` at line {1}")]
    UnexpectedOperator(String, usize),
}
