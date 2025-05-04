use thiserror::Error;

#[derive(Error, Debug)]
pub enum JacocoError {
    #[error("Invalid header. Expected: 0xC0C0, got {0:#X}")]
    WrongMagicHeader(i16),
    #[error("Invalid format version. Expected: 0x1007, got: {0:#X}")]
    WrongFormatVersion(i16),
    #[error("Invalid block type. Expected one of the following: 0x01, 0x10, 0x11, got: {0:#x}")]
    WrongBlockType(u8),
    #[error("Invalid execution data file")]
    InvalidFile,
    #[error("Invalid unix timestamp: {0}")]
    InvalidTimestamp(i64),
    #[error("Different ids: ({0:016x} and {1:016x})")]
    IllegalStateDifferentIds(i64, i64),
    #[error("Different class names {0} and {1} for id {2}")]
    IllegalStateDifferentNames(String, String, i64),
    #[error("Incompatible execution data for class {0} with id {1:016x}")]
    IllegalStateIncompatibleProbes(String, i64),
}
