use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use quad::Quad;

use crate::logic::world::ChunkData;

mod quad;

pub struct WorldRenderer;

impl Plugin for WorldRenderer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_mesh);
    }
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: Query<(Entity, &ChunkData), Without<Handle<Mesh>>>,
) {
    for (chunk_id, chunk_data) in &mut chunks {
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut colors = Vec::<[f32; 4]>::new();
        let mut indices = Vec::<u32>::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if chunk_data.blocks[x + y * 16 + z * 16 * 16] == 0 {
                        continue;
                    }

                    let (x, y, z) = (x as i32, y as i32, z as i32);

                    let mut forward = Quad::from_direction(
                        quad::Direction::Forward,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut forward.vertices);
                    colors.append(&mut forward.colors);
                    indices.append(&mut forward.indices);

                    let mut back = Quad::from_direction(
                        quad::Direction::Back,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut back.vertices);
                    colors.append(&mut back.colors);
                    indices.append(&mut back.indices);

                    let mut left = Quad::from_direction(
                        quad::Direction::Left,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut left.vertices);
                    colors.append(&mut left.colors);
                    indices.append(&mut left.indices);

                    let mut right = Quad::from_direction(
                        quad::Direction::Right,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut right.vertices);
                    colors.append(&mut right.colors);
                    indices.append(&mut right.indices);

                    let mut top = Quad::from_direction(
                        quad::Direction::Up,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut top.vertices);
                    colors.append(&mut top.colors);
                    indices.append(&mut top.indices);

                    let mut bottom = Quad::from_direction(
                        quad::Direction::Down,
                        vertices.len(),
                        IVec3::new(x, y, z),
                        Color::srgb(0.0, 1.0, 0.0),
                    );

                    vertices.append(&mut bottom.vertices);
                    colors.append(&mut bottom.colors);
                    indices.append(&mut bottom.indices);
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

        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            ..Default::default()
        });

        commands.entity(chunk_id).insert(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material,
            transform: Transform::from_xyz(
                16.0 * chunk_data.x as f32,
                16.0 * chunk_data.y as f32,
                16.0 * chunk_data.z as f32,
            ),
            ..default()
        });
    }
}
