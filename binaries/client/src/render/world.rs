use ::voxel::world::{chunk::Chunk, VoxelWorld};
use bevy::prelude::*;
use voxel::ChunkMaterial;

pub mod chunk;
pub mod voxel;

pub struct VoxelWorldRenderer;

impl Plugin for VoxelWorldRenderer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ChunkMaterial>::default());
        app.add_systems(Update, generate_chunk_mesh);
    }
}

pub fn generate_chunk_mesh(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ChunkMaterial>>,
    chunks: Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    world: Query<&VoxelWorld>,
) {
    if let Err(error) = chunk::generate_chunk_mesh(commands, meshes, materials, chunks, world) {
        eprintln!("{}", error)
    }
}
