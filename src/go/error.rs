use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoCoverageError {
    #[error("Invalid mode string")]
    InvalidMode,

    #[error("Submitted invalid mode: {0}")]
    InvalidModeName(String),

    #[error("Invalid line: {0:?}")]
    InvalidLine(String),
}
