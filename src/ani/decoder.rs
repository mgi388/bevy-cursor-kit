use std::{
    fmt,
    io::{Error as IoError, Read, Seek},
};

use byteorder::{LittleEndian, ReadBytesExt};
use ico::IconDir;
use riff::{Chunk, ChunkId, LIST_ID};

use super::*;

#[derive(Debug)]
pub enum DecodeError {
    IoError(IoError),
    UnsupportedRootChunkId(ChunkId),
    UnsupportedRootType(ChunkId),
    MissingHeaderChunk,
    MissingFramesChunk,
    UnsupportedFrameChunkId(ChunkId),
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
            DecodeError::UnsupportedRootChunkId(id) => {
                write!(f, "unsupported root chunk ID: {:?} (expected 'RIFF')", id)
            }
            DecodeError::UnsupportedRootType(id) => {
                write!(f, "unsupported root type: {:?} (expected 'ACON')", id)
            }
            DecodeError::MissingHeaderChunk => write!(f, "missing header chunk ('anih')"),
            DecodeError::MissingFramesChunk => write!(f, "missing frames chunk ('fram')"),
            DecodeError::UnsupportedFrameChunkId(id) => {
                write!(f, "unsupported frame chunk ID: {:?} (expected 'icon')", id)
            }
        }
    }
}

pub struct Decoder<R>
where
    R: Read + Seek,
{
    reader: R,
}

fn read_chunks<T>(iter: &mut riff::Iter<T>) -> std::io::Result<Vec<Chunk>>
where
    T: Read + Seek,
{
    let mut vec: Vec<Chunk> = Vec::new();
    for item in iter {
        match item {
            Ok(chunk) => vec.push(chunk),
            Err(e) => return Err(e),
        }
    }
    Ok(vec)
}

impl<R: Read + Seek> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }

    pub fn decode(&mut self) -> Result<AnimatedCursor, DecodeError> {
        const fn chunk_id(value: &[u8; 4]) -> ChunkId {
            ChunkId { value: *value }
        }

        let chunk = riff::Chunk::read(&mut self.reader, 0)?;

        if chunk.id() != riff::RIFF_ID {
            return Err(DecodeError::UnsupportedRootChunkId(chunk.id()));
        }

        let type_chunk_id = chunk.read_type(&mut self.reader)?;
        if type_chunk_id != chunk_id(b"ACON") {
            return Err(DecodeError::UnsupportedRootType(type_chunk_id));
        }

        let chunks = read_chunks(&mut chunk.iter(&mut self.reader))?;

        let metadata: Result<AnimatedCursorMetadata, DecodeError> = chunks
            .iter()
            .find(|c| c.id() == chunk_id(b"anih"))
            .map(|c| {
                let contents = c.read_contents(&mut self.reader)?;
                let mut cursor = std::io::Cursor::new(contents);

                Ok(AnimatedCursorMetadata {
                    header_size_bytes: cursor.read_u32::<LittleEndian>()?,
                    frame_count: cursor.read_u32::<LittleEndian>()?,
                    step_count: cursor.read_u32::<LittleEndian>()?,
                    width: cursor.read_u32::<LittleEndian>()?,
                    height: cursor.read_u32::<LittleEndian>()?,
                    bit_count: cursor.read_u32::<LittleEndian>()?,
                    plane_count: cursor.read_u32::<LittleEndian>()?,
                    frames_per_60_seconds: cursor.read_u32::<LittleEndian>()?,
                    flags: cursor.read_u32::<LittleEndian>()?,
                })
            })
            .ok_or(DecodeError::MissingHeaderChunk)?;

        let metadata = metadata?;

        let frames = chunks
            .iter()
            .find(|c| c.id() == LIST_ID)
            .map(|c| {
                if c.read_type(&mut self.reader)? != chunk_id(b"fram") {
                    return Err(DecodeError::MissingFramesChunk);
                }

                read_chunks(&mut c.iter(&mut self.reader))?
                    .iter()
                    .map(|c| {
                        if c.id() != chunk_id(b"icon") {
                            return Err(DecodeError::UnsupportedFrameChunkId(c.id()));
                        };

                        let contents = c.read_contents(&mut self.reader)?;

                        let icon = IconDir::read(&mut std::io::Cursor::new(contents))?;

                        Ok(icon)
                    })
                    .collect()
            })
            .transpose()?
            .ok_or(DecodeError::MissingFramesChunk)?;

        Ok(AnimatedCursor { metadata, frames })
    }
}
