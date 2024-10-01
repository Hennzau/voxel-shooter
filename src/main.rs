use bevy::{color::palettes::css::WHITE, prelude::*, utils::HashMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

use logic::{
    player::{Player, PlayerFocus, PlayerManagement},
    utilities::CursorGrabber,
    world::{WorldData, WorldLogic},
};
use render::world::WorldRenderer;

#[derive(Debug, Component)]
pub struct MainPlayer;

mod logic;
mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CursorGrabber, PlayerManagement, WorldLogic, WorldRenderer))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::srgb(0.72, 1.0, 0.98)))
        .add_systems(Update, focus_player)
        .add_systems(Startup, (setup, construct_world))
        .run();
}

fn focus_player(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<Entity, With<MainPlayer>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for player in &mut players {
            commands.entity(player).insert(PlayerFocus);
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        for player in &mut players {
            commands.entity(player).remove::<PlayerFocus>();
        }
    }
}

fn construct_world(mut commands: Commands) {
    commands
        .spawn(WorldData::new())
        .insert(Name::new("World"))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 700.0,
    });
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    let mut player = commands.spawn((Player, MainPlayer));

    player.insert(Camera3dBundle {
        projection: PerspectiveProjection {
            near: 0.1,
            far: 100.0,
            fov: 70.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    player.insert(Name::new("Player"));
}
