use std::time::Duration;

use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor},
};
use bevy_cursor_kit::{ani::animation::AnimationDuration, prelude::*};
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
        .add_systems(Update, (insert_cursor, animate_cursor))
        .run();
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Debug, Resource)]
struct Cursors {
    cursor: Handle<AnimatedCursor>,
}

fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cursors {
        cursor: asset_server.load("Master Sword-Fairy.ANI"),
    });
}

fn insert_cursor(
    mut commands: Commands,
    animated_cursors: Res<Assets<AnimatedCursor>>,
    cursors: Res<Cursors>,
    window: Single<Entity, With<Window>>,
    mut setup: Local<bool>,
) {
    if *setup {
        return;
    }

    let Some(c) = animated_cursors.get(&cursors.cursor.clone()) else {
        return;
    };

    commands.entity(*window).insert((
        CursorIcon::Custom(CustomCursorImageBuilder::from_animated_cursor(c, None).build()),
        c.hotspots.clone(),
        AnimationConfig::new(
            0,
            c.animation.clips[0].atlas_indices.len() - 1,
            match c.animation.clips[0].duration {
                AnimationDuration::PerFrame(millis) => Duration::from_millis(millis as u64),
                AnimationDuration::PerRepetition(_) => panic!("PerRepetition not supported"),
            },
        ),
    ));

    *setup = true;
}

#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    duration_per_frame: Duration,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, duration_per_frame: Duration) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            duration_per_frame,
            frame_timer: Timer::new(duration_per_frame, TimerMode::Once),
        }
    }
}

/// This system loops through all the sprites in the [`CursorIcon`]'s
/// [`TextureAtlas`], from [`AnimationConfig`]'s `first_sprite_index` to
/// `last_sprite_index`.
fn animate_cursor(
    time: Res<Time>,
    mut query: Query<(&mut CursorIcon, &CursorHotspots, &mut AnimationConfig)>,
) {
    for (mut cursor_icon, hotspots, mut config) in &mut query {
        let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon else {
            continue;
        };

        config.frame_timer.tick(time.delta());

        if !config.frame_timer.finished() {
            continue;
        }

        let Some(atlas) = image.texture_atlas.as_mut() else {
            continue;
        };

        let mut new_index = atlas.index + 1;

        if new_index > config.last_sprite_index {
            new_index = config.first_sprite_index;
        }

        if new_index != atlas.index {
            atlas.index = new_index;

            info!("Changed to sprite index {}", atlas.index);
        }

        config.frame_timer = Timer::new(config.duration_per_frame, TimerMode::Once);

        // Animation frames may have different hotspots, so we need to update
        // the hotspot for each frame.
        let new_hotspot = hotspots.get_or_default(atlas.index);
        if new_hotspot != image.hotspot {
            image.hotspot = new_hotspot;

            info!("Changed to hotspot {:?}", image.hotspot);
        }
    }
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
