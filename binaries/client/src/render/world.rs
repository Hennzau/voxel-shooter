use ::voxel::world::{
    chunk::{Chunk, ChunkUpdated},
    VoxelWorld,
};
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
    images: ResMut<Assets<Image>>,
    chunks: Query<(&Parent, Entity, &Chunk), Without<Handle<Mesh>>>,
    all_chunks: Query<&Chunk>,
    world: Query<&VoxelWorld>,
) {
    if let Err(error) = chunk::generate_chunk_mesh(
        commands, meshes, materials, images, chunks, all_chunks, world,
    ) {
        eprintln!("{}", error)
    }
}

// pub fn update_chunk_mesh(
//     commands: Commands,
//     meshes: ResMut<Assets<Mesh>>,
//     chunks: Query<(&Parent, Entity, &Handle<Mesh>, &Chunk), With<ChunkUpdated>>,
//     all_chunks: Query<&Chunk>,
//     world: Query<&VoxelWorld>,
// ) {
//     if let Err(error) = chunk::update_chunk_mesh(commands, meshes, chunks, all_chunks, world) {
//         eprintln!("{}", error)
//     }
// }
