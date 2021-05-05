#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unterminated comment")]
    UnterminatedComment
}
