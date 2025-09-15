use super::go::GoCoverageError;
use crate::jacoco::JacocoError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Jacoco(JacocoError),

    #[error(transparent)]
    Go(GoCoverageError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl From<JacocoError> for Error {
    fn from(value: JacocoError) -> Self {
        Self::Jacoco(value)
    }
}

impl From<GoCoverageError> for Error {
    fn from(value: GoCoverageError) -> Self {
        Self::Go(value)
    }
}
