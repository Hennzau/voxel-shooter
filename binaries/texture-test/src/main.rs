use bevy::{
    color::palettes::css::WHITE,
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, Texture, TextureDimension, TextureFormat},
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use cursor::CursorGrabber;
use logic::player::{Player, PlayerFocus, PlayerManagement};
use quad::{ChunkMaterial, Quad, ATTRIBUTE_VOXEL};

pub mod cursor;
pub mod quad;

#[derive(Debug, Component)]
pub struct MainPlayer;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((CursorGrabber, PlayerManagement))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(MaterialPlugin::<ChunkMaterial>::default())
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

fn construct_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Quad {
        mut vertices,
        mut indices,
        ..
    } = Quad::from_direction(quad::Direction::Front, 0, IVec3::ZERO, 0, 15);

    let Quad {
        vertices: vertices2,
        indices: indices2,
        ..
    } = Quad::from_direction(
        quad::Direction::Front,
        vertices.len(),
        IVec3::new(1, 0, 0),
        0,
        15,
    );

    vertices.extend(vertices2);
    indices.extend(indices2);

    let Quad {
        vertices: vertices3,
        indices: indices3,
        ..
    } = Quad::from_direction(
        quad::Direction::Front,
        vertices.len(),
        IVec3::new(0, 1, 0),
        0,
        15,
    );

    vertices.extend(vertices3);
    indices.extend(indices3);

    let Quad {
        vertices: vertices4,
        indices: indices4,
        ..
    } = Quad::from_direction(
        quad::Direction::Front,
        vertices.len(),
        IVec3::new(1, 1, 0),
        0,
        15,
    );

    vertices.extend(vertices4);
    indices.extend(indices4);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(ATTRIBUTE_VOXEL, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    let image = Image::new(
        Extent3d {
            width: 2,
            height: 2,
            depth_or_array_layers: 2,
        },
        TextureDimension::D3,
        vec![0, 1, 2, 3, 4, 5, 6, 7],
        TextureFormat::R8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let material = materials.add(ChunkMaterial {
        image_3d: images.add(image),
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material,
        ..default()
    });

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
