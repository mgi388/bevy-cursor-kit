use bevy_app::prelude::*;
use bevy_asset::{io::Reader, prelude::*, AssetLoader, LoadContext, RenderAssetUsages};
use bevy_image::{Image, TextureAtlasBuilder, TextureAtlasBuilderError, TextureAtlasLayout};
use bevy_reflect::prelude::*;
use ico::ResourceType;
use image::{DynamicImage, ImageBuffer};
use thiserror::Error;

use crate::{
    ani::{
        decoder::{DecodeError, Decoder},
        AnimatedCursorMetadata,
    },
    hotspot::CursorHotspots,
};

use super::animation::*;
#[cfg(feature = "serde_json_asset")]
use super::serde_asset::JsonDeserializer;
#[cfg(feature = "serde_ron_asset")]
use super::serde_asset::RonDeserializer;
#[cfg(feature = "serde_asset")]
use super::serde_asset::SerdeAnimatedCursorAssetPlugin;
#[cfg(feature = "serde_toml_asset")]
use super::serde_asset::TomlDeserializer;

pub struct AnimatedCursorAssetPlugin;

impl Plugin for AnimatedCursorAssetPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "serde_asset")]
        {
            #[cfg(feature = "serde_json_asset")]
            if !app.is_plugin_added::<SerdeAnimatedCursorAssetPlugin<JsonDeserializer>>() {
                app.add_plugins(SerdeAnimatedCursorAssetPlugin::<JsonDeserializer>::new(
                    ["ANI.json", "ani.json"].to_vec(),
                ));
            }
            #[cfg(feature = "serde_ron_asset")]
            if !app.is_plugin_added::<SerdeAnimatedCursorAssetPlugin<RonDeserializer>>() {
                app.add_plugins(SerdeAnimatedCursorAssetPlugin::<RonDeserializer>::new(
                    ["ANI.ron", "ani.ron"].to_vec(),
                ));
            }
            #[cfg(feature = "serde_toml_asset")]
            if !app.is_plugin_added::<SerdeAnimatedCursorAssetPlugin<TomlDeserializer>>() {
                app.add_plugins(SerdeAnimatedCursorAssetPlugin::<TomlDeserializer>::new(
                    ["ANI.toml", "ani.toml"].to_vec(),
                ));
            }
        }

        app.init_asset::<AnimatedCursor>()
            .init_asset_loader::<AnimatedCursorLoader>()
            .register_asset_reflect::<AnimatedCursor>();
    }
}

#[derive(Asset, Clone, Debug, Reflect)]
#[reflect(Debug)]
pub struct AnimatedCursor {
    /// The metadata for the animated cursor. This is optional and only set for
    /// .ANI files.
    pub(super) metadata: Option<AnimatedCursorMetadata>,
    /// A handle to the image asset.
    pub image: Handle<Image>,
    /// A handle to the texture atlas layout asset.
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    /// The hotspot data.
    pub hotspots: CursorHotspots,
    /// The animation to play.
    pub animation: Animation,
}

impl AnimatedCursor {
    /// Returns the hotspot for the cursor at the given index, or `(0, 0)` if
    /// the index is out of bounds.
    #[inline(always)]
    pub fn hotspot_or_default(&self, index: usize) -> (u16, u16) {
        self.hotspots.get_or_default(index)
    }
}

/// A loader for animated cursor assets from .ANI files.
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Debug, Default)]
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
                        load_context.labeled_asset_scope(
                            format!("image_{}", i).to_string(),
                            |_| -> Result<Image, AnimatedCursorLoaderError> { Ok(image.clone()) },
                        )?,
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

        let texture_atlas_layout = load_context.labeled_asset_scope(
            "texture_atlas_layout".to_string(),
            |_| -> Result<TextureAtlasLayout, AnimatedCursorLoaderError> {
                Ok(texture_atlas_layout)
            },
        )?;
        let image = load_context.labeled_asset_scope(
            "image".to_string(),
            |_| -> Result<Image, AnimatedCursorLoaderError> { Ok(image) },
        )?;

        // Convert the hotspots to a `CursorHotspots` struct. The `overrides`
        // are constructed to include an entry for every frame. This means that
        // the `default` hotspot is never actually used. We could optimize by
        // checking for the most common hotspot and using that as the default,
        // but that's probably not worth the effort.
        let hotspots = CursorHotspots {
            overrides: hotspots
                .iter()
                .enumerate()
                .map(|(i, hotspot)| (i, *hotspot))
                .collect(),
            ..Default::default()
        };

        Ok(AnimatedCursor {
            metadata: Some(c.metadata.clone()),
            image,
            texture_atlas_layout,
            hotspots,
            animation: Animation {
                clips: vec![AnimationClip {
                    atlas_indices: (0..c.metadata.frame_count as usize).collect(),
                    duration: AnimationDuration::PerFrame(
                        c.metadata.duration_per_frame().as_millis() as u32,
                    ),
                    direction: AnimationDirection::Forwards,
                }],
                repeat: AnimationRepeat::Loop,
                direction: AnimationDirection::Forwards,
            },
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ANI", "ani"]
    }
}
