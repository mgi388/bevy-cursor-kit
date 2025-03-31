use std::path::PathBuf;

use bevy_asset::{AssetPath, LoadContext, RenderAssetUsages};
use bevy_image::prelude::*;
use image::{DynamicImage, GenericImage as _, GenericImageView as _, Rgba};
use thiserror::Error;

/// Errors that can occur when loading an image.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LoadImageError {
    /// An error occurred while loading the image.
    #[error("could not load image: {dependency}")]
    Error { dependency: AssetPath<'static> },
}

/// Loads an image from `path` as an [`Image`] and returns it.
///
/// If `color_key` is provided, pixels in the image with the same color as the
/// key are made transparent.
///
/// If `flip_x` is `true`, the image is flipped horizontally.
///
/// If `flip_y` is `true`, the image is flipped vertically.
pub(crate) async fn load_image(
    load_context: &mut LoadContext<'_>,
    path: &str,
    color_key: Option<(u8, u8, u8)>,
    flip_x: bool,
    flip_y: bool,
) -> Result<Image, LoadImageError> {
    let path: PathBuf = path.into();

    let loaded = load_context
        .loader()
        .immediate()
        .load::<Image>(path.clone())
        .await
        .map_err(|_| LoadImageError::Error {
            dependency: path.clone().into(),
        })?;

    let img = loaded.get();

    let mut dyn_img = img
        .clone()
        .try_into_dynamic()
        .map_err(|_| LoadImageError::Error {
            dependency: path.clone().into(),
        })?;

    dyn_img = process_image(dyn_img, color_key, flip_x, flip_y);

    Ok(Image::from_dynamic(
        dyn_img,
        true,
        RenderAssetUsages::default(),
    ))
}

fn process_image(
    src_img: DynamicImage,
    color_key: Option<(u8, u8, u8)>,
    flip_x: bool,
    flip_y: bool,
) -> DynamicImage {
    let (width, height) = src_img.dimensions();

    let mut dest_img = DynamicImage::new_rgba8(width, height);

    for y in 0..height {
        for x in 0..width {
            let target_x = if flip_x { width - 1 - x } else { x };
            let target_y = if flip_y { height - 1 - y } else { y };

            let mut pixel = src_img.get_pixel(x, y);

            // Apply color keying if a color key is provided.
            if let Some((r, g, b)) = color_key {
                if pixel[0] == r && pixel[1] == g && pixel[2] == b {
                    pixel = Rgba([0, 0, 0, 0]); // make pixel transparent
                }
            }

            dest_img.put_pixel(target_x, target_y, pixel);
        }
    }

    dest_img
}
