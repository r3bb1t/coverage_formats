use super::{BlockType, Error, ExecutionData, JacocoError, JacocoReport, Result, SessionInfo};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Local};
use std::io::{ErrorKind, Read};

impl JacocoReport {
    pub fn from_read<R: Read>(r: &mut R) -> Result<Self> {
        let mut is_first_block = false;

        let mut session_infos = vec![];
        let mut execution_datas = vec![];

        loop {
            let Ok(i) = Self::read(r) else { break };
            let block_type = BlockType::try_from(i)?;

            if is_first_block && block_type != BlockType::Header {
                return Err(JacocoError::InvalidFile.into());
            }
            is_first_block = false;

            if let Err(Error::Io(io_error)) =
                Self::read_block(r, block_type, &mut session_infos, &mut execution_datas)
            {
                if io_error.kind() == ErrorKind::UnexpectedEof {
                    break;
                }
            }
        }

        let jacoco_report = Self {
            session_infos,
            execution_datas,
        };
        Ok(jacoco_report)
    }

    fn read_block<R: Read>(
        r: &mut R,
        block_type: BlockType,
        session_infos: &mut Vec<SessionInfo>,
        execution_datas: &mut Vec<ExecutionData>,
    ) -> Result<()> {
        match block_type {
            BlockType::Header => {
                Self::read_header(r)?;
            }
            BlockType::SessionInfo => {
                let session_info = Self::read_session_info(r)?;
                session_infos.push(session_info);
            }
            BlockType::ExecutionData => {
                let execution_data = Self::read_execution_data(r)?;
                execution_datas.push(execution_data);
            }
        };

        Ok(())
    }

    fn read_header<R: Read>(r: &mut R) -> Result<()> {
        let first_char = Self::read_char(r)?;
        if first_char != Self::MAGIC_NUMBER {
            return Err(JacocoError::WrongMagicHeader(first_char).into());
        }

        let second_char = Self::read_char(r)?;
        if second_char != Self::FORMAT_VERSION {
            return Err(JacocoError::WrongFormatVersion(second_char).into());
        }

        Ok(())
    }

    fn read_session_info<R: Read>(r: &mut R) -> Result<SessionInfo> {
        let id = Self::read_utf8(r)?;
        // FIXME: Dont unwrap
        let start_unix_timestamp = Self::read_long(r)?;
        let dump_unix_timestamp = Self::read_long(r)?;

        let start: DateTime<Local> = DateTime::from_timestamp_millis(start_unix_timestamp)
            .ok_or(JacocoError::InvalidTimestamp(start_unix_timestamp))?
            .into();

        let dump: DateTime<Local> = DateTime::from_timestamp_millis(dump_unix_timestamp)
            .ok_or(JacocoError::InvalidTimestamp(start_unix_timestamp))?
            .into();

        let session_info = SessionInfo { id, start, dump };

        Ok(session_info)
    }

    fn read_execution_data<R: Read>(r: &mut R) -> Result<ExecutionData> {
        let id = Self::read_long(r)?;
        let name = Self::read_utf8(r)?;
        let probes = Self::read_boolean_array(r)?;

        let execution_data = ExecutionData { id, name, probes };
        Ok(execution_data)
    }

    pub fn read_boolean_array<R: Read>(r: &mut R) -> std::io::Result<Vec<bool>> {
        let length = Self::read_var_int(r)? as usize;
        let mut value = Vec::with_capacity(length);
        let mut buffer = 0;
        for i in 0..length {
            if i % 8 == 0 {
                buffer = r.read_i8()?; // Read a new byte
            }
            value.push((buffer & 0x01) != 0);
            buffer >>= 1; // Shift right
        }
        Ok(value)
    }

    pub fn read_var_int<R: Read>(r: &mut R) -> std::io::Result<i32> {
        let mut value = r.read_i8()? as i32;
        if (value & 0x80) == 0 {
            return Ok(value);
        }
        value = (value & 0x7F) | (Self::read_var_int(r)? << 7);
        Ok(value)
    }

    fn read<R: Read>(r: &mut R) -> std::io::Result<i8> {
        r.read_i8()
    }

    fn read_char<R: Read>(r: &mut R) -> std::io::Result<i16> {
        r.read_i16::<BigEndian>()
    }

    fn read_long<R: Read>(r: &mut R) -> std::io::Result<i64> {
        r.read_i64::<BigEndian>()
    }

    fn read_utf8<R: Read>(r: &mut R) -> Result<String> {
        let length = r.read_u16::<BigEndian>()?;
        let mut full_string_buffer = vec![0; length.into()];
        r.read_exact(&mut full_string_buffer)?;

        let utf8_string = String::from_utf8(full_string_buffer)?;
        Ok(utf8_string)
    }
}
