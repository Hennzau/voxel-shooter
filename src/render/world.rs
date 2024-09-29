use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use quad::Quad;

use crate::logic::world::ChunkData;

mod quad;

#[derive(Debug, Component)]
struct ChunkMesh {
    mesh: Handle<Mesh>,
}

pub struct WorldRenderer;

impl Plugin for WorldRenderer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_mesh);
    }
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: Query<(Entity, &ChunkData), Without<ChunkMesh>>,
) {
    for (chunk_id, chunk_data) in &mut chunks {
        let mut vertices = Vec::<f32>::new();
        let mut colors = Vec::<f32>::new();
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

        commands.entity(chunk_id).insert(ChunkMesh {
            mesh: meshes.add(mesh),
        });
    }
}
