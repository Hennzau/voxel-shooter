use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use culler::CulledMesh;
use voxel::world::{
    chunk::{Chunk, ChunkNeighbors, ChunkUpdated, CHUNK_SIZE},
    VoxelWorld,
};

use super::voxel::{ChunkMaterial, ATTRIBUTE_VOXEL};

pub mod culler;

fn get_chunk<'a>(
    chunks: &'a Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    entity: Option<Entity>,
) -> Option<&'a Chunk> {
    match entity {
        Some(entity) => chunks.get(entity).map(|(_, _, c)| c).ok(),
        None => None,
    }
}

pub fn generate_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    chunks: Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    world: Query<&VoxelWorld>,
) -> eyre::Result<()> {
    for (parent, chunk_id, chunk) in &chunks {
        if let Ok(world) = world.get(parent.get()) {
            let ChunkNeighbors {
                left,
                right,
                top,
                bottom,
                front,
                back,
            } = world.neighbours(chunk);

            let (left, right, top, bottom, front, back) = (
                get_chunk(&chunks, left),
                get_chunk(&chunks, right),
                get_chunk(&chunks, top),
                get_chunk(&chunks, bottom),
                get_chunk(&chunks, front),
                get_chunk(&chunks, back),
            );

            let CulledMesh { vertices, indices } =
                CulledMesh::new(chunk, left, right, bottom, top, back, front)?;

            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );

            mesh.insert_attribute(ATTRIBUTE_VOXEL, vertices);
            mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

            let material = materials.add(ChunkMaterial {});

            commands.entity(chunk_id).insert(MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material,
                transform: Transform::from_xyz(
                    CHUNK_SIZE as f32 * chunk.pos.x as f32,
                    CHUNK_SIZE as f32 * chunk.pos.y as f32,
                    CHUNK_SIZE as f32 * chunk.pos.z as f32,
                ),
                ..default()
            });
        }
    }

    Ok(())
}
