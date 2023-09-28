use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ArelError(#[from] arel::Error),
    // #[error("error message: `{0}`")]
    // Message(String),
}
