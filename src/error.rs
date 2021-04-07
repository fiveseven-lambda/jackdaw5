#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unexpected character `{0}` at `{1}`")]
    UnexpectedCharacter(char, usize),
    #[error("single ampersand `&` at {0}; use `&&` instead")]
    SingleAmpersand(usize),
}
