use super::Result;
use crate::go::{GoCoverageError, GoProfileBlock, GoProfileMode, GoReport};

use lazy_regex::lazy_regex;
use std::{io::BufRead, str::FromStr};

impl GoReport {
    pub fn from_buf_read<R: BufRead>(r: &mut R) -> Result<Self> {
        // 12 since normally we expect first line to be not longer than "mode: atomic" + newline
        let mut first_line = String::with_capacity(13);
        r.read_line(&mut first_line)?;
        let first_line_re = lazy_regex!("^mode: (.*)\n$");
        let captures = first_line_re
            .captures(&first_line)
            .ok_or(GoCoverageError::InvalidMode)?;

        let (_, [mode_str]) = captures.extract();

        let mode = GoProfileMode::from_str(mode_str)?;

        let mut profile_blocks = vec![];
        let mut current_line = String::new();
        while r.read_line(&mut current_line).is_ok() {
            if current_line.is_empty() {
                break; // EOF
            }
            let parsed_profile_block = GoProfileBlock::from_str(current_line.trim_end())?;
            profile_blocks.push(parsed_profile_block);
            current_line.clear();
        }

        let report = Self {
            mode,
            profile_blocks,
        };

        Ok(report)
    }
}
