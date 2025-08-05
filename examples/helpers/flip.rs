use bevy::{
    prelude::*,
    window::{CursorIcon, CustomCursor},
};

pub struct FlipPlugin;

impl Plugin for FlipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_flip_x, toggle_flip_y));
    }
}

fn toggle_flip_x(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CursorIcon, With<Window>>,
) {
    if input.just_pressed(KeyCode::KeyX) {
        for mut cursor_icon in &mut query {
            if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
                image.flip_x = !image.flip_x;
            }
        }
    }
}

fn toggle_flip_y(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CursorIcon, With<Window>>,
) {
    if input.just_pressed(KeyCode::KeyY) {
        for mut cursor_icon in &mut query {
            if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon {
                image.flip_y = !image.flip_y;
            }
        }
    }
}
