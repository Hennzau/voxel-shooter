use bevy::prelude::*;

use super::blocks::Block;

pub const CHUNK_SIZE: usize = 15;

#[derive(Debug, Component)]
pub struct Chunk {
    pub pos: IVec3,

    // Store the blocks in a flat array
    pub blocks: [u8; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    pub healths: [u8; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],

    // Mask to determine if a block is solid for fast face culling
    pub x_axis: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub y_axis: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub z_axis: [u16; CHUNK_SIZE * CHUNK_SIZE],
}

pub struct ChunkMask {
    pub left: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub right: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub bottom: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub top: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub back: [u16; CHUNK_SIZE * CHUNK_SIZE],
    pub front: [u16; CHUNK_SIZE * CHUNK_SIZE],
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
            blocks: [Block::Air.as_u8(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            healths: [15; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            x_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            y_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            z_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn blocks_as_u8(&self) -> Vec<u8> {
        self.blocks.into()
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> eyre::Result<Block> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!(format!("Index {:?} out of bounds", (x, y, z))));
        }

        Ok(Block::from_u8(
            self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE],
        ))
    }

    pub fn get_health(&self, x: usize, y: usize, z: usize) -> eyre::Result<u8> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!(format!("Index {:?} out of bounds", (x, y, z))));
        }

        Ok(self.healths[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE])
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
        self.healths[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] = health;

        self.x_axis[y + z * CHUNK_SIZE] |= 1 << x;
        self.y_axis[x + z * CHUNK_SIZE] |= 1 << y;
        self.z_axis[x + y * CHUNK_SIZE] |= 1 << z;

        Ok(())
    }
}
