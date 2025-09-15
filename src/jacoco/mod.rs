pub(super) use super::{Error, Result};
pub use error::JacocoError;

use chrono::{DateTime, Datelike, Local};
use core::fmt::Display;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

mod error;
mod reader;

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct JacocoReport {
    session_infos: Vec<SessionInfo>,
    execution_datas: Vec<ExecutionData>,
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BlockType {
    Header = 0x01,
    SessionInfo = 0x10,
    ExecutionData = 0x11,
}

/// Data object describing a session which was the source of execution data.
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SessionInfo {
    /// arbitrary session identifier
    id: String,
    /// the epoc based time stamp when execution data recording has been started
    start: DateTime<Local>,
    /// the epoc based time stamp when execution data was collected
    dump: DateTime<Local>,
}

/// Execution data for a single Java class. While instances are immutable care
/// has to be taken about the probe data array of type `Vec<bool>`
/// which can be modified.
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExecutionData {
    /// class identifier
    id: i64,
    /// VM name
    name: String,
    /// probe data
    probes: Vec<bool>,
}

impl JacocoReport {
    pub fn new(session_infos: Vec<SessionInfo>, execution_datas: Vec<ExecutionData>) -> Self {
        Self {
            session_infos,
            execution_datas,
        }
    }

    pub fn session_infos(&self) -> &Vec<SessionInfo> {
        &self.session_infos
    }

    pub fn session_infos_mut(&mut self) -> &Vec<SessionInfo> {
        &mut self.session_infos
    }

    pub fn execution_datas(&self) -> &Vec<ExecutionData> {
        &self.execution_datas
    }

    pub fn execution_datas_mut(&mut self) -> &Vec<ExecutionData> {
        &mut self.execution_datas
    }
}

impl ExecutionData {
    pub fn new(id: i64, name: String, probes: Vec<bool>) -> Self {
        Self { id, name, probes }
    }

    pub fn id(&self) -> &i64 {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn probes(&self) -> &Vec<bool> {
        &self.probes
    }

    pub fn covered_lines(&self) -> usize {
        self.probes().iter().filter(|probe| probe == &&true).count()
    }
}

impl SessionInfo {
    pub fn new(id: String, start: DateTime<Local>, dump: DateTime<Local>) -> Self {
        Self { id, start, dump }
    }
}

impl JacocoReport {
    /// Magic number in header for file format identification.
    pub(super) const MAGIC_NUMBER: i16 = 0xC0C0u16 as i16;

    ///// Block identifier for file headers.
    //pub(super) const BLOCK_HEADER: i8 = 0x01;
    //
    ///// Block identifier for session information.
    //pub(super) const BLOCK_SESSIONINFO: i8 = 0x10;
    //
    ///// Block identifier for execution data of a single class.
    //pub(super) const BLOCK_EXECUTIONDATA: i8 = 0x11;

    pub(super) const FORMAT_VERSION: i16 = 0x1007;
}

impl ExecutionData {
    pub fn try_merge(self, other: Self) -> core::result::Result<Self, JacocoError> {
        if self.id != other.id {
            return Err(JacocoError::IllegalStateDifferentIds(self.id, other.id));
        }

        if self.name != other.name {
            return Err(JacocoError::IllegalStateDifferentNames(
                self.name, other.name, self.id,
            ));
        }

        if self.probes.len() != other.probes().len() {
            return Err(JacocoError::IllegalStateIncompatibleProbes(
                self.name, self.id,
            ));
        }

        let probes: Vec<bool> = self
            .probes
            .iter()
            .zip(other.probes.iter())
            .map(|(&old, &new)| old || new)
            .collect();

        Ok(Self {
            id: self.id,
            name: self.name,
            probes,
        })
    }
}

impl TryFrom<u8> for BlockType {
    type Error = JacocoError;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Header),
            0x10 => Ok(Self::SessionInfo),
            0x11 => Ok(Self::ExecutionData),
            _ => Err(JacocoError::WrongBlockType(value)),
        }
    }
}

impl Display for JacocoReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timezone_str_option = if let Ok(tz) = tz::TimeZone::local() {
            if let Ok(time_ty) = tz.find_current_local_time_type() {
                Some(time_ty.time_zone_designation().to_string())
            } else {
                None
            }
        } else {
            None
        };

        writeln!(f, "CLASS ID         HITS/PROBES   CLASS NAME")?;
        for session in &self.session_infos {
            let id = &session.id;
            let start_formatted = session.start.format("%a %b %d %H:%M:%S").to_string();
            let dump_formatted = session.dump.format("%a %b %d %H:%M:%S").to_string();

            let start_timezone_str = timezone_str_option
                .clone()
                .unwrap_or(session.start.format("%Z").to_string());
            let dump_timezone_str = timezone_str_option
                .clone()
                .unwrap_or(session.dump.format("%Z").to_string());

            let start_final = format!(
                "{start_formatted} {start_timezone_str} {}",
                session.start.year()
            );

            let dump_final = format!(
                "{dump_formatted} {dump_timezone_str} {}",
                session.dump.year()
            );

            writeln!(f, r#"Session "{id}": {start_final} - {dump_final}"#)?
        }

        for execution_data in &self.execution_datas {
            let id = execution_data.id;
            let covered_lines = execution_data.covered_lines();
            let probes_len = execution_data.probes.len();
            let name = &execution_data.name;

            writeln!(f, "{id:016x}  {covered_lines:3} of {probes_len:3}   {name}",)?
        }
        Ok(())
    }
}
