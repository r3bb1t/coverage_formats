use crate::go::GoReport;

use std::fmt::Display;

impl Display for GoReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mode: {}", self.mode.as_str())?;
        for super::GoProfileBlock {
            filename,
            start_line,
            start_col,
            end_line,
            end_col,
            number_of_statements,
            count,
        } in &self.blocks
        {
            writeln!(
                f,
                "{filename}:{start_line}.{start_col},{end_line}.{end_col} {number_of_statements} {count}"
            )?
        }
        Ok(())
    }
}
