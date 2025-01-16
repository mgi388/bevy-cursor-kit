//! Illustrates how to use the .cur.ron file format to load a static cursor.
//! This is a text equivalent of the classic .CUR binary format.
//!
//! Notice that the cursors you toggle between have different hotspots. The
//! crosshair has a hotspot at the center, while the arrow has a hotspot at the
//! top. You can verify this by hovering over the button and observing that the
//! hover state changes when these points are over the button.
use bevy::{
    color::palettes::basic::*,
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};
use bevy_cursor_kit::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorAssetPlugin)
        .add_systems(Startup, (setup_cursor, setup_ui, setup_instructions))
        .add_systems(Update, (insert_cursor, toggle_cursor))
        .add_systems(Update, button_system)
        .run();
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Debug, Resource)]
struct Cursors {
    static_cursor: Handle<StaticCursor>,
}

fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cursors {
        static_cursor: asset_server.load("kenney_crosshairPack.cur.ron"),
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

    let texture_atlas_index = 11; // a crosshair

    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: c.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: c.texture_atlas_layout.clone(),
                index: texture_atlas_index,
            }),
            flip_x: false,
            flip_y: false,
            rect: None,
            hotspot: c.hotspot_or_default(texture_atlas_index),
        }));

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

    let Some(c) = static_cursors.get(&cursors.static_cursor.clone()) else {
        return;
    };

    let new_texture_atlas_index = 95; // an arrow pointing up

    for mut cursor_icon in &mut query {
        if let CursorIcon::Custom(CustomCursor::Image {
            ref mut texture_atlas,
            ref mut hotspot,
            ..
        }) = *cursor_icon
        {
            if cached_texture_atlas_index_and_hotspot.is_none() {
                *cached_texture_atlas_index_and_hotspot =
                    Some((texture_atlas.as_ref().unwrap().index, *hotspot));
                *texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas.as_ref().unwrap().layout.clone(),
                    index: new_texture_atlas_index,
                });
                *hotspot = c.hotspot_or_default(new_texture_atlas_index);
            } else {
                let (cached_index, cached_hotspot) =
                    cached_texture_atlas_index_and_hotspot.unwrap();
                *texture_atlas = Some(TextureAtlas {
                    layout: texture_atlas.as_ref().unwrap().layout.clone(),
                    index: cached_index,
                });
                *hotspot = cached_hotspot;
                *cached_texture_atlas_index_and_hotspot = None;
            }
        }
    }
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Press C to toggle the cursor"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

// The following is just UI code, so we have a button to interact with.

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
