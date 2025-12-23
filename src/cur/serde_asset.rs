#[cfg(feature = "serde_toml_asset")]
use std::str::from_utf8;
use std::{fmt::Debug, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_asset::{io::Reader, prelude::*, AssetLoader, LoadContext};
use bevy_image::TextureAtlasLayout;
use bevy_math::UVec2;
use bevy_reflect::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    asset_image::{load_image, LoadImageError},
    hotspot::CursorHotspots,
};

use super::asset::StaticCursor;

/// A plugin for loading static cursor assets using Serde.
pub struct SerdeStaticCursorAssetPlugin<D: Deserializer> {
    _phantom: PhantomData<D>,

    extensions: Vec<&'static str>,
}

impl<D: Deserializer> SerdeStaticCursorAssetPlugin<D> {
    /// Creates a new [`SerdeStaticCursorAssetPlugin`].
    pub fn new(extensions: Vec<&'static str>) -> Self {
        Self {
            _phantom: PhantomData,
            extensions,
        }
    }
}

impl<D: Deserializer + TypePath> Plugin for SerdeStaticCursorAssetPlugin<D> {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(SerdeStaticCursorLoader::<D>::new(
            D::default(),
            self.extensions.clone(),
        ));
    }
}

#[derive(Asset, Debug, Clone, Deserialize, Reflect, Serialize)]
#[reflect(Debug, Deserialize, Serialize)]
pub struct SerdeStaticCursor {
    /// The image to use.
    pub image: SerdeImage,
    /// The layout of the texture atlas.
    pub texture_atlas_layout: SerdeTextureAtlasLayout,
    /// The hotspot data.
    #[serde(default)]
    pub hotspots: CursorHotspots,
}

#[derive(Clone, Debug, Default, Deserialize, Reflect, Serialize)]
#[reflect(Debug, Default, Deserialize, Serialize)]
pub struct SerdeImage {
    /// The path to the image asset relative to the assets root directory.
    pub path: String,
    /// An optional color key. Pixels in the image with this color are converted
    /// to transparent.
    #[serde(default)]
    pub color_key: Option<(u8, u8, u8)>,
    /// Whether to flip the image horizontally. Flips the entire image.
    #[serde(default)]
    pub flip_x: bool,
    /// Whether to flip the image vertically. Flips the entire image.
    #[serde(default)]
    pub flip_y: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Reflect, Serialize)]
#[reflect(Debug, Default, Deserialize, Serialize)]
pub struct SerdeTextureAtlasLayout {
    /// The size of each tile, in pixels.
    pub tile_size: UVec2,
    /// The number columns on the sprite sheet.
    pub columns: u32,
    /// The number of rows on the sprite sheet.
    pub rows: u32,
    /// The padding between each tile, in pixels.
    pub padding: Option<UVec2>,
    /// The global offset of the grid, in pixels.
    pub offset: Option<UVec2>,
}

/// Possible errors that can be produced by deserialization.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// A [serde_json::error::Error] error.
    #[cfg(feature = "serde_json_asset")]
    #[error("Could not parse the JSON: {0}")]
    Json(#[from] serde_json::error::Error),
    /// A [ron::error::SpannedError] error.
    #[cfg(feature = "serde_ron_asset")]
    #[error("could not parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
    /// A [std::str::Utf8Error] error.
    #[cfg(feature = "serde_toml_asset")]
    #[error("Could not interpret as UTF-8: {0}")]
    FormatError(#[from] std::str::Utf8Error),
    /// A [toml::de::Error] error.
    #[cfg(feature = "serde_toml_asset")]
    #[error("Could not parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
}

/// A trait for deserializing bytes into a [`SerdeStaticCursor`].
pub trait Deserializer: Debug + Default + Send + Sync + 'static {
    fn deserialize(&self, bytes: &[u8]) -> Result<SerdeStaticCursor, DeserializeError>;
}

/// Implements deserialization for JSON format.
#[cfg(feature = "serde_json_asset")]
#[derive(Clone, Debug, Default, TypePath)]
pub struct JsonDeserializer;

#[cfg(feature = "serde_json_asset")]
impl Deserializer for JsonDeserializer {
    fn deserialize(&self, bytes: &[u8]) -> Result<SerdeStaticCursor, DeserializeError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

/// Implements deserialization for RON format.
#[cfg(feature = "serde_ron_asset")]
#[derive(Clone, Debug, Default, TypePath)]
pub struct RonDeserializer;

#[cfg(feature = "serde_ron_asset")]
impl Deserializer for RonDeserializer {
    fn deserialize(&self, bytes: &[u8]) -> Result<SerdeStaticCursor, DeserializeError> {
        Ok(ron::de::from_bytes::<SerdeStaticCursor>(bytes)?)
    }
}

/// Implements deserialization for TOML format.
#[cfg(feature = "serde_toml_asset")]
#[derive(Clone, Debug, Default, TypePath)]
pub struct TomlDeserializer;

#[cfg(feature = "serde_toml_asset")]
impl Deserializer for TomlDeserializer {
    fn deserialize(&self, bytes: &[u8]) -> Result<SerdeStaticCursor, DeserializeError> {
        Ok(toml::from_str::<SerdeStaticCursor>(from_utf8(bytes)?)?)
    }
}

/// A loader for static cursor assets using Serde.
#[derive(TypePath)]
pub struct SerdeStaticCursorLoader<D: Deserializer + TypePath> {
    _phantom: PhantomData<D>,
    extensions: Vec<&'static str>,
    deserializer: D,
}

/// Possible errors that can be produced by [`SerdeStaticCursorLoader`].
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SerdeStaticCursorLoaderError {
    /// An [IO](std::io) error.
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [DeserializeError] error.
    #[error("could not deserialize static cursor: {0}")]
    DeserializeError(#[from] DeserializeError),
    /// A [LoadImageError] error.
    #[error("could not load image: {0}")]
    LoadImageError(#[from] LoadImageError),
}

impl<D: Deserializer + TypePath> AssetLoader for SerdeStaticCursorLoader<D> {
    type Asset = StaticCursor;
    type Settings = ();
    type Error = SerdeStaticCursorLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let c = self.deserializer.deserialize(&bytes)?;

        // Load the image asset. If the image has a color key or needs to be
        // flipped, load it as a dynamic image so we can process it now.
        // Otherwise, load it as a regular asset.
        let image = if c.image.color_key.is_some() || c.image.flip_x || c.image.flip_y {
            let image = load_image(
                load_context,
                &c.image.path,
                c.image.color_key,
                c.image.flip_x,
                c.image.flip_y,
            )
            .await?;
            load_context.add_labeled_asset("image".to_string(), image)
        } else {
            load_context.load(&c.image.path)
        };

        let texture_atlas_layout = bevy_image::TextureAtlasLayout::from_grid(
            c.texture_atlas_layout.tile_size,
            c.texture_atlas_layout.columns,
            c.texture_atlas_layout.rows,
            c.texture_atlas_layout.padding,
            c.texture_atlas_layout.offset,
        );

        let texture_atlas_layout = load_context.labeled_asset_scope(
            "texture_atlas_layout".to_string(),
            |_| -> Result<TextureAtlasLayout, SerdeStaticCursorLoaderError> {
                Ok(texture_atlas_layout)
            },
        )?;

        Ok(StaticCursor {
            image,
            texture_atlas_layout,
            hotspots: c.hotspots,
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

impl<D: Deserializer + TypePath> SerdeStaticCursorLoader<D> {
    pub fn new(deserializer: D, extensions: Vec<&'static str>) -> Self {
        Self {
            _phantom: PhantomData,
            deserializer,
            extensions,
        }
    }
}
