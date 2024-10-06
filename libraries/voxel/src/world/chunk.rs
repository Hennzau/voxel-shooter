use bevy::prelude::*;

use super::blocks::Block;

pub const CHUNK_SIZE: usize = 31;

#[derive(Debug, Component)]
pub struct Chunk {
    pub pos: IVec3,

    // Store the blocks in a flat array : 4bits for the block type and 4bits for the block health
    pub blocks: [u8; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],

    // Mask to determine if a block is solid for fast face culling
    pub x_axis: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub y_axis: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub z_axis: [u32; CHUNK_SIZE * CHUNK_SIZE],
}

pub struct ChunkMask {
    pub left: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub right: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub bottom: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub top: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub back: [u32; CHUNK_SIZE * CHUNK_SIZE],
    pub front: [u32; CHUNK_SIZE * CHUNK_SIZE],
}

impl ChunkMask {
    pub fn new() -> Self {
        Self {
            left: [0; CHUNK_SIZE * CHUNK_SIZE],
            right: [0; CHUNK_SIZE * CHUNK_SIZE],
            bottom: [0; CHUNK_SIZE * CHUNK_SIZE],
            top: [0; CHUNK_SIZE * CHUNK_SIZE],
            back: [0; CHUNK_SIZE * CHUNK_SIZE],
            front: [0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}

#[derive(Debug, Component)]
pub struct ChunkModification {
    pub blocks: Vec<(UVec3, Block, u8)>,
}

impl ChunkModification {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

#[derive(Debug, Component)]
pub struct TerrainGenerated;

#[derive(Debug, Component)]
pub struct VegetationGenerated;

pub struct ChunkNeighbors {
    pub left: Option<Entity>,
    pub right: Option<Entity>,
    pub bottom: Option<Entity>,
    pub top: Option<Entity>,
    pub back: Option<Entity>,
    pub front: Option<Entity>,
}

#[derive(Debug, Component)]
pub struct ChunkUpdated; // "Event" to notify that the chunk has been updated

impl Chunk {
    pub fn new(pos: IVec3) -> Self {
        Self {
            pos,
            blocks: [15 << 4 | Block::Air.as_u8(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            x_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            y_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            z_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn blocks(&self) -> Vec<u8> {
        self.blocks.into()
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> eyre::Result<Block> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!(format!("Index {:?} out of bounds", (x, y, z))));
        }

        Ok(Block::from(
            self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] & 0b1111,
        ))
    }

    pub fn get_health(&self, x: usize, y: usize, z: usize) -> eyre::Result<u8> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!(format!("Index {:?} out of bounds", (x, y, z))));
        }

        Ok(self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] >> 4)
    }

    pub fn set_block(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        block: Block,
        health: u8,
    ) -> eyre::Result<()> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!(format!("Index {:?} out of bounds", (x, y, z))));
        }

        self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] = block.as_u8();
        self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] |= health << 4;

        self.x_axis[y + z * CHUNK_SIZE] |= 1 << x;
        self.y_axis[x + z * CHUNK_SIZE] |= 1 << y;
        self.z_axis[x + y * CHUNK_SIZE] |= 1 << z;

        Ok(())
    }
}
