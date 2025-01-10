use std::{
    fmt,
    io::{Error as IoError, Read, Seek},
};

use ico::IconDir;

use super::*;

#[derive(Debug)]
pub enum DecodeError {
    IoError(IoError),
}

impl std::error::Error for DecodeError {}

impl From<IoError> for DecodeError {
    fn from(error: IoError) -> Self {
        DecodeError::IoError(error)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

pub struct Decoder<R>
where
    R: Read + Seek,
{
    reader: R,
}

impl<R: Read + Seek> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }

    pub fn decode(&mut self) -> Result<StaticCursor, DecodeError> {
        let icon = IconDir::read(&mut self.reader)?;

        Ok(StaticCursor(icon))
    }
}
