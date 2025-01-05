use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};
use bevy_cursor_kit::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorAssetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, insert_cursor)
        .run();
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Debug, Resource)]
struct Cursors {
    static_cursor: Handle<StaticCursorAsset>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.insert_resource(Cursors {
        static_cursor: asset_server.load("Master Sword.CUR"),
    });
}

fn insert_cursor(
    mut commands: Commands,
    static_cursors: Res<Assets<StaticCursorAsset>>,
    cursors: Res<Cursors>,
    window: Single<Entity, With<Window>>,
    mut setup: Local<bool>,
) {
    if *setup {
        return;
    }

    let Some(c) = static_cursors.get(&cursors.static_cursor.clone()) else {
        return;
    };

    let texture_atlas_index = 0;

    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: c.image.clone(),
            // TODO: Update for > Bevy 0.15.
            //
            // texture_atlas: Some(TextureAtlas {
            //     layout: c.texture_atlas_layout.clone(),
            //     index: texture_atlas_index,
            // }),
            // flip_x: false,
            // flip_y: false,
            // rect: None,
            hotspot: c.hotspots[texture_atlas_index],
        }));

    *setup = true;
}
