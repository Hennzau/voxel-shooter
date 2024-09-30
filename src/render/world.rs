use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};
use chunk_culler::ChunkMesh;
use mesh::ChunkMaterial;
use quad::Quad;

use crate::logic::world::ChunkData;

mod chunk_culler;
mod mesh;
mod quad;

pub struct WorldRenderer;

impl Plugin for WorldRenderer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ChunkMaterial>::default());
        app.add_systems(Update, generate_mesh);
    }
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut chunks: Query<(Entity, &ChunkData), Without<Handle<Mesh>>>,
) {
    for (chunk_id, chunk_data) in &mut chunks {
        let ChunkMesh { vertices, indices } = ChunkMesh::from_chunk_data(chunk_data);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );

        mesh.insert_attribute(mesh::ATTRIBUTE_VOXEL, vertices);
        mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

        let material = materials.add(mesh::ChunkMaterial {});

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
