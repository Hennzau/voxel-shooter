use voxel::world::{
    blocks::Block,
    chunk::{Chunk, CHUNK_SIZE},
};

use crate::render::world::voxel::{Direction, Quad};

pub struct CulledMesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

impl CulledMesh {
    pub fn new(
        chunk: &Chunk,
        left: Option<&Chunk>,
        right: Option<&Chunk>,
        bottom: Option<&Chunk>,
        top: Option<&Chunk>,
        back: Option<&Chunk>,
        front: Option<&Chunk>,
    ) -> eyre::Result<Self> {
        let mut vertices = Vec::<u32>::new();
        let mut indices = Vec::<u32>::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z)?;
                    if block == Block::Air {
                        continue;
                    }

                    let mut quad = Quad::from_direction(
                        Direction::Forward,
                        vertices.len(),
                        bevy::math::IVec3::new(x as i32, y as i32, z as i32),
                        block as u32 - 1,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }
            }
        }

        Ok(Self { vertices, indices })
    }
}
