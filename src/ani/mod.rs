pub mod asset;
pub mod decoder;

use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ico::IconDir;

pub use decoder::{DecodeError, Decoder};

#[derive(Clone, Debug, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub struct AnimatedCursorMetadata {
    // The header size in bytes.
    header_size_bytes: u32,
    // The number of frames in the animation.
    pub frame_count: u32,
    // The number of steps in the animation. May include duplicate frames.
    // Equals `frame_count`, if no 'seq '-chunk is present.
    pub step_count: u32,
    // The frame width in pixels.
    pub width: u32,
    // The frame height in pixels.
    pub height: u32,
    // The number of bits/pixel. `color_depth = 2 * bit_count`.
    pub bit_count: u32,
    // The number of planes.
    pub plane_count: u32,
    // The number of frames per 60 seconds.
    pub frames_per_60_secs: u32,
    // The animation flags.
    //
    // TODO: Use bitflags.
    pub flags: u32,
}

#[derive(Clone, Debug, Reflect)]
#[reflect(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", reflect(Deserialize, Serialize))]
pub struct AnimatedCursor {
    pub metadata: AnimatedCursorMetadata,
    #[reflect(ignore)]
    pub frames: Vec<IconDir>,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use std::ffi::{OsStr, OsString};
    use std::{
        fs::File,
        io::Read as _,
        path::{Path, PathBuf},
    };

    use crate::ico::ResourceType;

    use super::*;

    fn patched_reader_from_file(file: File) -> std::io::Cursor<Vec<u8>> {
        let mut reader = std::io::BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).unwrap();

        // The Warhammer: Dark Omen .ANI files seem to incorrectly set the RIFF
        // chunk size to `File Size`, rather than `File Size - 8`. This means
        // that the `File Size` field is 8 bytes too large and the `riff` crate
        // fails to parse it. We need to correct this by setting the correct
        // size in the RIFF header. We already know we're only dealing with .ANI
        // files, so we just check the first 4 bytes of the file to make sure
        // it's a RIFF file, and we only change the chunk size if it exactly
        // matches the number of read bytes.
        if bytes.len() >= 8
            && &bytes[0..4] == b"RIFF"
            && u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) == bytes.len() as u32
        {
            let size_bytes = (bytes.len() as u32 - 8).to_le_bytes();
            bytes[4] = size_bytes[0];
            bytes[5] = size_bytes[1];
            bytes[6] = size_bytes[2];
            bytes[7] = size_bytes[3];
        }

        std::io::Cursor::new(bytes)
    }

    #[test]
    fn test_decode_arrow_ani() {
        let d: PathBuf = [
            std::env::var("DARKOMEN_PATH").unwrap().as_str(),
            "DARKOMEN",
            "GRAPHICS",
            "CURSORS",
            "ARROW.ANI",
        ]
        .iter()
        .collect();

        let file = File::open(d).unwrap();
        let reader = patched_reader_from_file(file);
        let cursor = Decoder::new(reader).decode().unwrap();

        assert_eq!(
            cursor.metadata,
            AnimatedCursorMetadata {
                header_size_bytes: 36,
                frame_count: 4,
                step_count: 4,
                width: 0,
                height: 0,
                bit_count: 0,
                plane_count: 0,
                frames_per_60_secs: 10,
                flags: 1,
            }
        );

        assert_eq!(cursor.frames.len(), 4);

        for frame in &cursor.frames {
            assert_eq!(frame.resource_type(), ResourceType::Cursor);
            assert_eq!(frame.entries().len(), 1);

            let entry = frame.entries().first().unwrap();
            assert_eq!(entry.resource_type(), ResourceType::Cursor);

            let icon_image = entry.decode().unwrap();
            assert_eq!(icon_image.width(), 32);
            assert_eq!(icon_image.height(), 32);
        }
    }

    #[test]
    fn test_decode_all() {
        let d: PathBuf = [
            std::env::var("DARKOMEN_PATH").unwrap().as_str(),
            "DARKOMEN",
            "GRAPHICS",
            "CURSORS",
        ]
        .iter()
        .collect();

        let root_output_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "decoded", "cursors"]
            .iter()
            .collect();

        std::fs::create_dir_all(&root_output_dir).unwrap();

        fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) {
            println!("Reading dir {:?}", dir.display());

            let mut paths = std::fs::read_dir(dir)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()
                .unwrap();

            paths.sort();

            for path in paths {
                if path.is_dir() {
                    visit_dirs(&path, cb);
                } else {
                    cb(&path);
                }
            }
        }

        visit_dirs(&d, &mut |path| {
            let Some(ext) = path.extension() else {
                return;
            };
            if ext.to_string_lossy().to_uppercase() != "ANI" {
                return;
            }

            println!("Decoding {:?}", path.file_name().unwrap());

            let file = File::open(path).unwrap();
            let reader = patched_reader_from_file(file);
            let cursor = Decoder::new(reader).decode().unwrap();

            for frame in &cursor.frames {
                assert_eq!(frame.resource_type(), ResourceType::Cursor);
                assert_eq!(frame.entries().len(), 1);

                let entry = frame.entries().first().unwrap();
                assert_eq!(entry.resource_type(), ResourceType::Cursor);

                let icon_image = entry.decode().unwrap();
                assert_eq!(icon_image.width(), 32);
                assert_eq!(icon_image.height(), 32);
            }

            #[cfg(feature = "serde")]
            {
                let parent_dir = path
                    .components()
                    .collect::<Vec<_>>()
                    .iter()
                    .rev()
                    .skip(1) // skip the file name
                    .take_while(|c| c.as_os_str() != "DARKOMEN")
                    .collect::<Vec<_>>()
                    .iter()
                    .rev()
                    .collect::<PathBuf>();
                let output_dir = root_output_dir.join(parent_dir);
                std::fs::create_dir_all(&output_dir).unwrap();

                let output_path = append_ext("ron", output_dir.join(path.file_name().unwrap()));
                let mut output_file = File::create(output_path).unwrap();
                ron::ser::to_writer_pretty(&mut output_file, &cursor, Default::default()).unwrap();
            }
        });
    }

    #[cfg(feature = "serde")]
    fn append_ext(ext: impl AsRef<OsStr>, path: PathBuf) -> PathBuf {
        let mut os_string: OsString = path.into();
        os_string.push(".");
        os_string.push(ext.as_ref());
        os_string.into()
    }
}
