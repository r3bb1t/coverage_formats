use std::str::FromStr;

pub(super) use super::{Error, Result};
pub mod error;
pub use error::GoCoverageError;

pub mod reader;

use lazy_regex::{Lazy, Regex, lazy_regex};

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum GoProfileMode {
    Set,
    Count,
    Atomic,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct GoReport {
    mode: GoProfileMode,
    profile_blocks: Vec<GoProfileBlock>,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct GoProfileBlock {
    filename: String,
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
    number_of_statements: u32,
    count: u32,
}

impl FromStr for GoProfileMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "set" => Ok(Self::Set),
            "count" => Ok(Self::Count),
            "atomic" => Ok(Self::Atomic),
            _ => Err(GoCoverageError::InvalidModeName(s.to_string()).into()),
        }
    }
}

static GO_PROFILE_BLOCK_RE: Lazy<Regex, fn() -> Regex> =
    lazy_regex!(r#"^(.+):([0-9]+)\.([0-9]+),([0-9]+)\.([0-9]+) ([0-9]+) ([0-9]+)$"#);

impl FromStr for GoProfileBlock {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = GO_PROFILE_BLOCK_RE
            .captures(s)
            .ok_or(error::GoCoverageError::InvalidLine(s.to_string()))?;

        let (
            _,
            [
                filename,
                start_line,
                start_col,
                end_line,
                end_col,
                number_of_statements,
                count,
            ],
        ) = captures.extract();

        let block = Self {
            filename: filename.to_string(),
            start_line: start_line.parse()?,
            start_col: start_col.parse()?,
            end_line: end_line.parse()?,
            end_col: end_col.parse()?,
            number_of_statements: number_of_statements.parse()?,
            count: count.parse()?,
        };

        Ok(block)
    }
}

impl GoReport {
    pub fn new(mode: GoProfileMode, profile_blocks: Vec<GoProfileBlock>) -> Self {
        Self {
            mode,
            profile_blocks,
        }
    }

    pub fn mode(&self) -> &GoProfileMode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut GoProfileMode {
        &mut self.mode
    }

    pub fn profile_blocks(&self) -> &Vec<GoProfileBlock> {
        &self.profile_blocks
    }
    pub fn profile_blocks_mut(&mut self) -> &mut Vec<GoProfileBlock> {
        &mut self.profile_blocks
    }
}

impl GoProfileBlock {
    pub fn new(
        filename: String,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
        number_of_statements: u32,
        count: u32,
    ) -> Self {
        Self {
            filename,
            start_line,
            start_col,
            end_line,
            end_col,
            number_of_statements,
            count,
        }
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn filename_mut(&mut self) -> &mut String {
        &mut self.filename
    }

    pub fn start_line(&self) -> u32 {
        self.start_line
    }

    pub fn start_line_mut(&mut self) -> &mut u32 {
        &mut self.start_line
    }

    pub fn start_col(&self) -> u32 {
        self.start_col
    }

    pub fn start_col_mut(&mut self) -> &mut u32 {
        &mut self.start_col
    }

    pub fn end_line(&self) -> u32 {
        self.end_line
    }

    pub fn end_line_mut(&mut self) -> &mut u32 {
        &mut self.end_line
    }

    pub fn end_col(&self) -> u32 {
        self.end_col
    }

    pub fn end_col_mut(&mut self) -> &mut u32 {
        &mut self.end_col
    }

    pub fn number_of_statements(&self) -> u32 {
        self.number_of_statements
    }

    pub fn number_of_statements_mut(&mut self) -> &mut u32 {
        &mut self.number_of_statements
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn count_mut(&mut self) -> &mut u32 {
        &mut self.count
    }
}
