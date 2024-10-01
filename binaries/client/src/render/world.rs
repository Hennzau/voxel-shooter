use bevy::prelude::*;
use voxel::ChunkMaterial;

pub mod chunk;
pub mod voxel;

pub struct WorldRenderer;

impl Plugin for WorldRenderer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ChunkMaterial>::default());
        app.add_systems(Update, chunk::generate_chunk_mesh);
    }
}
