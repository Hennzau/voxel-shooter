use bevy::prelude::*;
use voxel::world::{chunk::Chunk, VoxelWorld};

use super::voxel::ChunkMaterial;

pub fn generate_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut chunks: Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    mut world: Query<&VoxelWorld>,
) {
    // generate mesh for each chunk
}
