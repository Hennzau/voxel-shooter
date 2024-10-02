use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct PlayerManagement;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct PlayerFocus;

impl Plugin for PlayerManagement {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerFocus>>,
) {
    for mut transform in &mut query {
        for motion in mouse_motion.read() {
            let yaw = -motion.delta.x * 0.3 * time.delta_seconds();
            let pitch = -motion.delta.y * 0.2 * time.delta_seconds();

            transform.rotate_y(yaw);
            transform.rotate_local_x(pitch);
        }

        let mut new_position = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if keyboard_input.pressed(KeyCode::KeyW) {
            new_position += forward.as_vec3();
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            new_position -= forward.as_vec3();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            new_position -= right.as_vec3();
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            new_position += right.as_vec3();
        }

        if keyboard_input.pressed(KeyCode::Space) {
            new_position += Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            new_position -= Vec3::Y;
        }

        if let Some(new_position) = new_position.try_normalize() {
            transform.translation += new_position * 50.0 * time.delta_seconds();
        }
    }
}
