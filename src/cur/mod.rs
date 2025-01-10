pub mod asset;
pub mod decoder;

use ico::IconDir;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StaticCursor(pub IconDir);

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use std::ffi::{OsStr, OsString};
    use std::{
        fs::File,
        path::{Path, PathBuf},
    };

    use ico::ResourceType;

    use crate::cur::decoder::Decoder;

    #[test]
    fn test_decode_hand_cur() {
        let d: PathBuf = [
            std::env::var("DARKOMEN_PATH").unwrap().as_str(),
            "DARKOMEN",
            "GRAPHICS",
            "CURSORS",
            "HAND.CUR",
        ]
        .iter()
        .collect();

        let file = File::open(d).unwrap();
        let cursor = Decoder::new(file).decode().unwrap();

        assert_eq!(cursor.0.resource_type(), ResourceType::Cursor);
        assert_eq!(cursor.0.entries().len(), 1);

        for entry in cursor.0.entries().iter() {
            assert_eq!(entry.resource_type(), ResourceType::Cursor);

            let icon_image = entry.decode().unwrap();
            assert_eq!(icon_image.width(), 32);
            assert_eq!(icon_image.height(), 32);

            assert_eq!(icon_image.cursor_hotspot(), Some((10, 20)));
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

        let root_output_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "decoded", "static-cursors"]
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
            if ext.to_string_lossy().to_uppercase() != "CUR" {
                return;
            }

            println!("Decoding {:?}", path.file_name().unwrap());

            let file = File::open(path).unwrap();
            let cursor = Decoder::new(file).decode().unwrap();

            assert_eq!(cursor.0.resource_type(), ResourceType::Cursor);
            assert_eq!(cursor.0.entries().len(), 1);

            for entry in cursor.0.entries().iter() {
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
