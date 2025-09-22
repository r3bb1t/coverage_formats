use std::{collections::HashMap, str::FromStr};

pub(super) use super::{Error, Result};
pub mod error;
pub use error::GoCoverageError;

pub mod reader;
pub mod writer;

use lazy_regex::{lazy_regex, Lazy, Regex};

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GoProfileMode {
    Set,
    Count,
    Atomic,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GoReport {
    mode: GoProfileMode,
    blocks: Vec<GoProfileBlock>,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GoProfileBlock {
    filename: String,
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
    number_of_statements: u32,
    count: u32,
}

impl GoReport {
    /// # Errors
    ///
    /// Will return 'GoCoverageError::InconsistentNumStmt' if it encounters two coverages of the
    /// same block with different numbers of statements
    pub fn try_merge(self, other: Self) -> Result<Self> {
        let mode = [self.mode, other.mode]
            .into_iter()
            .find(|m| *m != GoProfileMode::Set)
            .unwrap_or(GoProfileMode::Set);

        type Key = (String, u32, u32, u32, u32);

        let mut map: HashMap<Key, GoProfileBlock> =
            HashMap::with_capacity(core::cmp::max(self.blocks.len(), other.blocks.len()));

        let make_key = |b: &GoProfileBlock| {
            (
                b.filename.clone(),
                b.start_line,
                b.start_col,
                b.end_line,
                b.end_col,
            )
        };

        // Insert all blocks from self
        for b in self.blocks {
            map.insert(make_key(&b), b);
        }

        // Merge non-zero blocks from other
        for b in other.blocks.into_iter().filter(|b| b.count != 0) {
            let k = make_key(&b);
            match map.entry(k) {
                std::collections::hash_map::Entry::Occupied(mut occ) => {
                    let existing = occ.get_mut();
                    if existing.number_of_statements != b.number_of_statements {
                        return Err(Error::Go(GoCoverageError::InconsistentNumStmt {
                            from: b.number_of_statements,
                            to: existing.number_of_statements,
                        }));
                    }
                    existing.count = match mode {
                        GoProfileMode::Set => existing.count | b.count,
                        GoProfileMode::Count | GoProfileMode::Atomic => {
                            existing.count.saturating_add(b.count)
                        }
                    };
                }
                std::collections::hash_map::Entry::Vacant(vac) => {
                    vac.insert(b);
                }
            }
        }

        let mut blocks: Vec<_> = map.into_values().collect();
        blocks.sort();

        Ok(GoReport { mode, blocks })
    }
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

impl GoProfileMode {
    fn as_str(&self) -> &'static str {
        match self {
            GoProfileMode::Set => "set",
            GoProfileMode::Count => "count",
            GoProfileMode::Atomic => "atomic",
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

        let (_, [filename, start_line, start_col, end_line, end_col, number_of_statements, count]) =
            captures.extract();

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
            blocks: profile_blocks,
        }
    }

    pub fn mode(&self) -> &GoProfileMode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut GoProfileMode {
        &mut self.mode
    }

    pub fn profile_blocks(&self) -> &Vec<GoProfileBlock> {
        &self.blocks
    }
    pub fn profile_blocks_mut(&mut self) -> &mut Vec<GoProfileBlock> {
        &mut self.blocks
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
