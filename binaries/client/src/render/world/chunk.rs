use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use greedy_mesher::GreedyMesh;
use voxel::world::{
    chunk::{Chunk, ChunkNeighbors, ChunkUpdated, CHUNK_SIZE},
    VoxelWorld,
};

use super::voxel::{ChunkMaterial, ATTRIBUTE_VOXEL};

pub mod culler;
pub mod greedy_mesher;

fn get_chunk<'a>(all_chunks: &'a Query<&Chunk>, entity: Option<Entity>) -> Option<&'a Chunk> {
    match entity {
        Some(entity) => all_chunks.get(entity).ok(),
        None => None,
    }
}

pub fn generate_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut images: ResMut<Assets<Image>>,
    chunks: Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    all_chunks: Query<&Chunk>,
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
            } = world.neighbours(chunk.pos);

            let (left, right, top, bottom, front, back) = (
                get_chunk(&all_chunks, left),
                get_chunk(&all_chunks, right),
                get_chunk(&all_chunks, top),
                get_chunk(&all_chunks, bottom),
                get_chunk(&all_chunks, front),
                get_chunk(&all_chunks, back),
            );

            let GreedyMesh { vertices, indices } =
                GreedyMesh::new(chunk, left, right, bottom, top, back, front)?;

            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );

            mesh.insert_attribute(ATTRIBUTE_VOXEL, vertices);
            mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

            let material = materials.add(ChunkMaterial::new(chunk, &mut images));

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

pub fn update_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut images: ResMut<Assets<Image>>,
    chunks: Query<
        (
            &Parent,
            Entity,
            &Handle<Mesh>,
            &Handle<ChunkMaterial>,
            &Chunk,
        ),
        With<ChunkUpdated>,
    >,
    all_chunks: Query<&Chunk>,
    world: Query<&VoxelWorld>,
) -> eyre::Result<()> {
    for (parent, chunk_id, mesh, material, chunk) in &chunks {
        if let Ok(world) = world.get(parent.get()) {
            let ChunkNeighbors {
                left,
                right,
                top,
                bottom,
                front,
                back,
            } = world.neighbours(chunk.pos);

            let (left, right, top, bottom, front, back) = (
                get_chunk(&all_chunks, left),
                get_chunk(&all_chunks, right),
                get_chunk(&all_chunks, top),
                get_chunk(&all_chunks, bottom),
                get_chunk(&all_chunks, front),
                get_chunk(&all_chunks, back),
            );

            let GreedyMesh { vertices, indices } =
                GreedyMesh::new(chunk, left, right, bottom, top, back, front)?;

            if let Some(mesh) = meshes.get_mut(mesh.id()) {
                mesh.remove_attribute(ATTRIBUTE_VOXEL);
                mesh.remove_indices();

                mesh.insert_attribute(ATTRIBUTE_VOXEL, vertices);
                mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
            }

            if let Some(material) = materials.get_mut(material.id()) {
                material.update(chunk, &mut images);
            }

            commands.entity(chunk_id).remove::<ChunkUpdated>();
        }
    }

    Ok(())
}
