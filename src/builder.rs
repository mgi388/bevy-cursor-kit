use bevy_asset::Handle;
use bevy_image::{Image, TextureAtlas};
use bevy_math::URect;
use bevy_reflect::prelude::*;
use bevy_winit::cursor::CustomCursor;

use crate::{ani::asset::AnimatedCursor, cur::asset::StaticCursor};

/// A builder for a [`CustomCursor::Image`].
#[derive(Debug, Default, Reflect)]
#[reflect(Debug, Default)]
pub struct CustomCursorImageBuilder {
    handle: Handle<Image>,
    texture_atlas: Option<TextureAtlas>,
    flip_x: bool,
    flip_y: bool,
    rect: Option<URect>,
    hotspot: (u16, u16),
}

impl CustomCursorImageBuilder {
    /// Create a builder from a [`StaticCursor`].
    ///
    /// The `index` parameter is used to select the texture atlas index to use.
    /// If `None`, index 0 is used.
    pub fn from_static_cursor(c: &StaticCursor, index: Option<usize>) -> Self {
        Self {
            handle: c.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: c.texture_atlas_layout.clone(),
                index: index.unwrap_or(0),
            }),
            hotspot: c.hotspot_or_default(index.unwrap_or(0)),
            ..Default::default()
        }
    }

    /// Create a builder from an [`AnimatedCursor`].
    ///
    /// The `index` parameter is used to select the texture atlas index to use.
    /// If `None`, index 0 is used.
    pub fn from_animated_cursor(c: &AnimatedCursor, index: Option<usize>) -> Self {
        Self {
            handle: c.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: c.texture_atlas_layout.clone(),
                index: index.unwrap_or(0),
            }),
            hotspot: c.hotspot_or_default(index.unwrap_or(0)),
            ..Default::default()
        }
    }

    /// Set the handle.
    pub fn handle(mut self, handle: Handle<Image>) -> Self {
        self.handle = handle;
        self
    }

    /// Set the texture atlas.
    pub fn texture_atlas(mut self, texture_atlas: Option<TextureAtlas>) -> Self {
        self.texture_atlas = texture_atlas;
        self
    }

    /// Set whether to flip horizontally.
    pub fn flip_x(mut self, flip_x: bool) -> Self {
        self.flip_x = flip_x;
        self
    }

    /// Set whether to flip vertically.
    pub fn flip_y(mut self, flip_y: bool) -> Self {
        self.flip_y = flip_y;
        self
    }

    /// Set the rectangle.
    pub fn rect(mut self, rect: URect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// Set the hotspot.
    pub fn hotspot(mut self, hotspot: (u16, u16)) -> Self {
        self.hotspot = hotspot;
        self
    }

    /// Build the `CustomCursor::Image`.
    pub fn build(self) -> CustomCursor {
        CustomCursor::Image {
            handle: self.handle,
            texture_atlas: self.texture_atlas,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            rect: self.rect,
            hotspot: self.hotspot,
        }
    }
}
