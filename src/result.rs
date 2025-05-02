use crate::jacoco::JacocoError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Jacoco(JacocoError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
}

impl From<JacocoError> for Error {
    fn from(value: JacocoError) -> Self {
        Self::Jacoco(value)
    }
}
