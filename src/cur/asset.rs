use bevy_app::prelude::*;
use bevy_asset::{io::Reader, prelude::*, AssetLoader, LoadContext, RenderAssetUsages};
use bevy_image::Image;
use bevy_reflect::prelude::*;
use bevy_sprite::{TextureAtlasBuilder, TextureAtlasBuilderError, TextureAtlasLayout};
use ico::ResourceType;
use image::{DynamicImage, ImageBuffer};
use thiserror::Error;

use crate::{
    cur::decoder::{DecodeError, Decoder},
    hotspot::CursorHotspots,
};

pub struct StaticCursorAssetPlugin;

impl Plugin for StaticCursorAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<StaticCursor>()
            .init_asset_loader::<StaticCursorLoader>()
            .register_asset_reflect::<StaticCursor>();
    }
}

#[derive(Asset, Clone, Debug, Reflect)]
#[reflect(Debug)]
pub struct StaticCursor {
    pub image: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    /// The hotspot data.
    pub hotspots: CursorHotspots,
}

impl StaticCursor {
    /// Returns the hotspot for the cursor at the given index, or `(0, 0)` if
    /// the index is out of bounds.
    ///
    /// Most .CUR files contain only one frame so this method is useful for
    /// getting the hotspot of the first frame without having to worry about the
    /// index being out of bounds.
    #[inline(always)]
    pub fn hotspot_or_default(&self, index: usize) -> (u16, u16) {
        self.hotspots.get_or_default(index)
    }
}

#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Debug, Default)]
pub struct StaticCursorLoader;

/// Possible errors that can be produced by [`StaticCursorLoader`].
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum StaticCursorLoaderError {
    /// An [IO](std::io) error.
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [DecodeError] error.
    #[error("could not decode static cursor: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("resource type must be cursor, found: {0}")]
    InvalidResourceType(String),
    #[error("missing hotspot")]
    MissingHotspot,
    #[error("could not create image buffer")]
    ImageBufferError,
    #[error("could not build texture atlas: {0}")]
    TextureAtlasBuilderError(#[from] TextureAtlasBuilderError),
}

impl AssetLoader for StaticCursorLoader {
    type Asset = StaticCursor;
    type Settings = ();
    type Error = StaticCursorLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let reader = std::io::Cursor::new(bytes);

        let mut decoder = Decoder::new(reader);

        let c = decoder.decode()?;

        let items =
            c.0.entries()
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    if e.resource_type() != ResourceType::Cursor {
                        return Err(StaticCursorLoaderError::InvalidResourceType(format!(
                            "{:?}",
                            e.resource_type()
                        )));
                    }

                    let icon_image = e.decode()?;

                    let image = ImageBuffer::from_raw(
                        icon_image.width(),
                        icon_image.height(),
                        icon_image.rgba_data().to_vec(),
                    )
                    .map(DynamicImage::ImageRgba8)
                    .ok_or(StaticCursorLoaderError::ImageBufferError)?;

                    let image = Image::from_dynamic(image, true, RenderAssetUsages::MAIN_WORLD);

                    let hotspot = icon_image
                        .cursor_hotspot()
                        .ok_or(StaticCursorLoaderError::MissingHotspot)?;

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
                .collect::<Result<Vec<_>, StaticCursorLoaderError>>()?;

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

        Ok(StaticCursor {
            image,
            texture_atlas_layout,
            hotspots,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["CUR", "cur"]
    }
}
