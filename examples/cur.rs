use bevy::{prelude::*, window::CursorIcon};
use bevy_cursor_kit::prelude::*;
use flip::FlipPlugin;
use ui::UiPlugin;

#[path = "./helpers/flip.rs"]
mod flip;
#[path = "./helpers/ui.rs"]
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorAssetPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(FlipPlugin)
        .add_systems(Startup, (setup_cursor, setup_instructions))
        .add_systems(Update, insert_cursor)
        .run();
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Debug, Resource)]
struct Cursors {
    cursor: Handle<StaticCursor>,
}

fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cursors {
        cursor: asset_server.load("Master Sword.CUR"),
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

    let Some(c) = static_cursors.get(&cursors.cursor.clone()) else {
        return;
    };

    commands.entity(*window).insert(CursorIcon::Custom(
        CustomCursorImageBuilder::from_static_cursor(c, None).build(),
    ));

    *setup = true;
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "Press X to toggle the cursor's `flip_x` setting\n
Press Y to toggle the cursor's `flip_y` setting",
        ),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
