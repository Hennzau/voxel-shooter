use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CursorGrabber;

impl Plugin for CursorGrabber {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cursor_grab);
    }
}

fn cursor_grab(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
