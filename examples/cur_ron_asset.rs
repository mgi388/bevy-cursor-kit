//! Illustrates how to use the .cur.ron file format to load a static cursor.
//! This is a text equivalent of the classic .CUR binary format.
//!
//! Notice that the cursors you toggle between have different hotspots. The
//! crosshair has a hotspot at the center, while the arrow has a hotspot at the
//! top. You can verify this by hovering over the button and observing that the
//! hover state changes when these points are over the button.
use bevy::{
    prelude::*,
    window::{CursorIcon, CustomCursor},
};
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
        .add_systems(Update, (insert_cursor, toggle_cursor))
        .run();
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Debug, Resource)]
struct Cursors {
    cursor: Handle<StaticCursor>,
}

fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cursors {
        cursor: asset_server.load("kenney_crosshairPack.cur.ron"),
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

    let texture_atlas_index = 11; // a crosshair

    commands.entity(*window).insert(CursorIcon::Custom(
        CustomCursorImageBuilder::from_static_cursor(c, Some(texture_atlas_index)).build(),
    ));

    *setup = true;
}

fn toggle_cursor(
    input: Res<ButtonInput<KeyCode>>,
    static_cursors: Res<Assets<StaticCursor>>,
    cursors: Res<Cursors>,
    mut query: Query<&mut CursorIcon, With<Window>>,
    mut cached_texture_atlas_index_and_hotspot: Local<Option<(usize, (u16, u16))>>, /* this lets us restore the previous value */
) {
    if !input.just_pressed(KeyCode::KeyC) {
        return;
    }

    let Some(c) = static_cursors.get(&cursors.cursor.clone()) else {
        return;
    };

    let new_texture_atlas_index = 95; // an arrow pointing up

    for mut cursor_icon in &mut query {
        if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
            if cached_texture_atlas_index_and_hotspot.is_none() {
                *cached_texture_atlas_index_and_hotspot =
                    Some((image.texture_atlas.as_ref().unwrap().index, image.hotspot));
                image.texture_atlas = Some(TextureAtlas {
                    layout: image.texture_atlas.as_ref().unwrap().layout.clone(),
                    index: new_texture_atlas_index,
                });
                image.hotspot = c.hotspot_or_default(new_texture_atlas_index);
            } else {
                let (cached_index, cached_hotspot) =
                    cached_texture_atlas_index_and_hotspot.unwrap();
                image.texture_atlas = Some(TextureAtlas {
                    layout: image.texture_atlas.as_ref().unwrap().layout.clone(),
                    index: cached_index,
                });
                image.hotspot = cached_hotspot;
                *cached_texture_atlas_index_and_hotspot = None;
            }
        }
    }
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "Press C to toggle the cursor\n
Press X to toggle the cursor's `flip_x` setting\n
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
