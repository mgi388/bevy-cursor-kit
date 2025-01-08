use std::time::Duration;

use bevy_app::prelude::*;
use bevy_asset::{io::Reader, prelude::*, AssetLoader, LoadContext, RenderAssetUsages};
use bevy_image::Image;
use bevy_reflect::prelude::*;
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasBuilderError, TextureAtlasLayout};
use image::{DynamicImage, ImageBuffer};
use thiserror::Error;

use crate::{
    ani::{
        decoder::{DecodeError, Decoder},
        AnimatedCursorMetadata,
    },
    ico::ResourceType,
};

pub struct AnimatedCursorAssetPlugin;

impl Plugin for AnimatedCursorAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimatedCursor>()
            .init_asset_loader::<AnimatedCursorLoader>()
            .register_asset_reflect::<AnimatedCursor>();
    }
}

#[derive(Asset, Clone, Debug, Reflect)]
#[reflect(Debug)]
pub struct AnimatedCursor {
    pub metadata: AnimatedCursorMetadata,
    pub image: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub hotspots: Vec<(u16, u16)>,
}

impl AnimatedCursor {
    /// Returns the hotspot for the cursor at the given index, or `(0, 0)` if
    /// the index is out of bounds.
    pub fn hotspot_or_default(&self, index: usize) -> (u16, u16) {
        self.hotspots.get(index).copied().unwrap_or((0, 0))
    }

    pub fn duration_per_frame(&self) -> Duration {
        Duration::from_secs_f32(self.metadata.ticks_per_frame as f32 / 60.0)
    }
}

#[derive(Clone, Default)]
pub struct AnimatedCursorLoader;

/// Possible errors that can be produced by [`AnimatedCursorLoader`].
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AnimatedCursorLoaderError {
    /// An [IO](std::io) error.
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [DecodeError] error.
    #[error("could not decode animated cursor: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("unsupported frame entry count: {0} (expected 1)")]
    UnsupportedFrameEntryCount(usize),
    #[error("resource type must be cursor")]
    InvalidResourceType,
    #[error("missing hotspot")]
    MissingHotspot,
    #[error("could not create image buffer")]
    ImageBufferError,
    #[error("could not build texture atlas: {0}")]
    TextureAtlasBuilderError(#[from] TextureAtlasBuilderError),
}

impl AssetLoader for AnimatedCursorLoader {
    type Asset = AnimatedCursor;
    type Settings = ();
    type Error = AnimatedCursorLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // The Warhammer: Dark Omen .ANI files seem to incorrectly set the RIFF
        // chunk size to `File Size`, rather than `File Size - 8`. This means
        // that the `File Size` field is 8 bytes too large and the `riff` crate
        // fails to parse it. We need to correct this by setting the correct
        // size in the RIFF header. We already know we're only dealing with .ANI
        // files, so we just check the first 4 bytes of the file to make sure
        // it's a RIFF file, and we only change the chunk size if it exactly
        // matches the number of read bytes.
        //
        // Warhammer: Dark Omen may have used a custom tool to create these .ANI
        // files, which is why they're not following the standard. There could
        // be other .ANI files out there that are also incorrectly formatted, so
        // it shouldn't hurt to check for this.
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

        let reader = std::io::Cursor::new(bytes);

        let mut decoder = Decoder::new(reader);

        let c = decoder.decode()?;

        let items = c
            .frames
            .iter()
            .enumerate()
            .map(|(i, f)| {
                if f.entries().len() != 1 {
                    return Err(AnimatedCursorLoaderError::UnsupportedFrameEntryCount(
                        f.entries().len(),
                    ));
                }

                let first = f.entries().first().unwrap();

                if first.resource_type() != ResourceType::Cursor {
                    return Err(AnimatedCursorLoaderError::InvalidResourceType);
                }

                let icon_image = first.decode()?;

                let image = ImageBuffer::from_raw(
                    icon_image.width(),
                    icon_image.height(),
                    icon_image.rgba_data().to_vec(),
                )
                .map(DynamicImage::ImageRgba8)
                .ok_or(AnimatedCursorLoaderError::ImageBufferError)?;

                let image = Image::from_dynamic(image, true, RenderAssetUsages::MAIN_WORLD);

                let hotspot = icon_image
                    .cursor_hotspot()
                    .ok_or(AnimatedCursorLoaderError::MissingHotspot)?;

                Ok((
                    (
                        load_context
                            .labeled_asset_scope(format!("image_{}", i).to_string(), |_| {
                                image.clone()
                            }),
                        image,
                    ),
                    hotspot,
                ))
            })
            .collect::<Result<Vec<_>, AnimatedCursorLoaderError>>()?;

        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        let mut hotspots = Vec::new();

        for ((handle, image), hotspot) in items.iter() {
            texture_atlas_builder.add_texture(Some(handle.id()), image);

            hotspots.push(*hotspot);
        }

        let (texture_atlas_layout, _, image) = texture_atlas_builder.build()?;

        let texture_atlas_layout = load_context
            .labeled_asset_scope("texture_atlas_layout".to_string(), |_| texture_atlas_layout);
        let image = load_context.labeled_asset_scope("image".to_string(), |_| image);

        Ok(AnimatedCursor {
            metadata: c.metadata,
            image,
            texture_atlas_layout,
            hotspots,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ANI", "ani"]
    }
}
