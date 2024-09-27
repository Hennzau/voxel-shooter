use bevy::{
    color::palettes::css::{ORANGE_RED, WHITE},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ],
    );

    mesh.insert_indices(bevy::render::mesh::Indices::U32(vec![0, 2, 1]));

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

    // camera
    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection {
            near: 0.1,
            far: 100.0,
            fov: 70.0,
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
