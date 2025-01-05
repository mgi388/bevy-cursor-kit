use bevy_app::prelude::*;

use crate::{ani::asset::AnimatedCursorAssetPlugin, cur::asset::StaticCursorAssetPlugin};

pub mod ani;
mod cur;
pub mod ico;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{ani::asset::*, cur::asset::*, CursorAssetPlugin};
}

pub struct CursorAssetPlugin;

impl Plugin for CursorAssetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<StaticCursorAssetPlugin>() {
            app.add_plugins(StaticCursorAssetPlugin);
        }
        if !app.is_plugin_added::<AnimatedCursorAssetPlugin>() {
            app.add_plugins(AnimatedCursorAssetPlugin);
        }
    }
}
