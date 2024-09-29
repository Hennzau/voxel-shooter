use bevy::{
    color::palettes::css::WHITE,
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use logic::{
    player::{Player, PlayerFocus, PlayerManagement},
    utilities::CursorGrabber,
};

#[derive(Debug, Component)]
pub struct MainPlayer;

mod logic;
mod render;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorGrabber, PlayerManagement))
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

fn construct_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut vertices = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    let mut index = 0;

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                // 8 vertices per cube

                vertices.push([x as f32, y as f32, z as f32]);
                vertices.push([x as f32 + 1.0, y as f32, z as f32]);
                vertices.push([x as f32 + 1.0, y as f32 + 1.0, z as f32]);
                vertices.push([x as f32, y as f32 + 1.0, z as f32]);
                vertices.push([x as f32, y as f32, z as f32 + 1.0]);
                vertices.push([x as f32 + 1.0, y as f32, z as f32 + 1.0]);
                vertices.push([x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0]);
                vertices.push([x as f32, y as f32 + 1.0, z as f32 + 1.0]);

                // Push good green color for each vertex

                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);
                colors.push([0.0, 1.0, 0.0, 1.0]);

                // 12 triangles per cube, push indices at once

                indices.push(index + 1);
                indices.push(index + 0);
                indices.push(index + 2);
                indices.push(index + 2);
                indices.push(index + 0);
                indices.push(index + 3);

                indices.push(index + 4);
                indices.push(index + 5);
                indices.push(index + 6);
                indices.push(index + 4);
                indices.push(index + 6);
                indices.push(index + 7);

                indices.push(index + 0);
                indices.push(index + 4);
                indices.push(index + 7);
                indices.push(index + 0);
                indices.push(index + 7);
                indices.push(index + 3);

                indices.push(index + 1);
                indices.push(index + 2);
                indices.push(index + 6);
                indices.push(index + 1);
                indices.push(index + 6);
                indices.push(index + 5);

                indices.push(index + 0);
                indices.push(index + 1);
                indices.push(index + 5);
                indices.push(index + 0);
                indices.push(index + 5);
                indices.push(index + 4);

                indices.push(index + 2);
                indices.push(index + 3);
                indices.push(index + 7);
                indices.push(index + 2);
                indices.push(index + 7);
                indices.push(index + 6);

                index += 8;
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let mesh = meshes.add(mesh);
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 500.0,
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
}
