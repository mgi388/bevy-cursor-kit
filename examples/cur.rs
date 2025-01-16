use bevy::{prelude::*, winit::cursor::CursorIcon};
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
    static_cursor: Handle<StaticCursor>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.insert_resource(Cursors {
        static_cursor: asset_server.load("Master Sword.CUR"),
    });
}

fn insert_cursor(
    mut commands: Commands,
    static_cursors: Res<Assets<StaticCursor>>,
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

    commands.entity(*window).insert(CursorIcon::Custom(
        CustomCursorImageBuilder::from_static_cursor(c, None).build(),
    ));

    *setup = true;
}
