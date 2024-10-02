use bevy::prelude::*;

use super::blocks::Block;

pub const CHUNK_SIZE: usize = 15;

#[derive(Debug, Component)]
pub struct Chunk {
    pub pos: IVec3,

    // Store the blocks in a flat array
    pub blocks: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],

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
            blocks: [Block::Air; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            x_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            y_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
            z_axis: [0b0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> eyre::Result<Block> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!("Index out of bounds"));
        }

        Ok(self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE])
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) -> eyre::Result<()> {
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Err(eyre::eyre!("Index out of bounds"));
        }

        self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] = block;

        self.x_axis[y + z * CHUNK_SIZE] |= 1 << x;
        self.y_axis[x + z * CHUNK_SIZE] |= 1 << y;
        self.z_axis[x + y * CHUNK_SIZE] |= 1 << z;

        Ok(())
    }
    /*
    pub fn get_solid(
        &self,
        left: Option<&Self>,
        right: Option<&Self>,
        bottom: Option<&Self>,
        top: Option<&Self>,
        back: Option<&Self>,
        front: Option<&Self>,
    ) -> eyre::Result<ChunkMask> {
        let mut mask = ChunkMask::new();

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x_axis = self.x_axis[i + j * CHUNK_SIZE];
                let y_axis = self.y_axis[i + j * CHUNK_SIZE];
                let z_axis = self.z_axis[i + j * CHUNK_SIZE];

                let mask_left = match left {
                    Some(chunk) => match chunk.get_block(CHUNK_SIZE - 1, i, j)? != Block::Air {
                        true => !(x_axis << 1 | ((1 << 1) - 1)) & x_axis,
                        false => !(x_axis << 1) & x_axis,
                    },
                    None => !(x_axis << 1) & x_axis,
                };

                mask.left[i + j * CHUNK_SIZE] = mask_left;

                let mask_right = match right {
                    Some(chunk) => match chunk.get_block(0, i, j)? != Block::Air {
                        true => !(x_axis >> 1 | (!0 << 31)) & x_axis,
                        false => !(x_axis >> 1) & x_axis,
                    },
                    None => !(x_axis >> 1) & x_axis,
                };

                mask.right[i + j * CHUNK_SIZE] = mask_right;

                let mask_bottom = match bottom {
                    Some(chunk) => match chunk.get_block(i, CHUNK_SIZE - 1, j)? != Block::Air {
                        true => !(y_axis << 1 | ((1 << 1) - 1)) & y_axis,
                        false => !(y_axis << 1) & y_axis,
                    },
                    None => !(y_axis << 1) & y_axis,
                };

                mask.bottom[i + j * CHUNK_SIZE] = mask_bottom;

                let mask_top = match top {
                    Some(chunk) => match chunk.get_block(i, 0, j)? != Block::Air {
                        true => !(y_axis >> 1 | (!0 << 31)) & y_axis,
                        false => !(y_axis >> 1) & y_axis,
                    },
                    None => !(y_axis >> 1) & y_axis,
                };

                mask.top[i + j * CHUNK_SIZE] = mask_top;

                let mask_back = match back {
                    Some(chunk) => match chunk.get_block(i, j, CHUNK_SIZE - 1)? != Block::Air {
                        true => !(z_axis << 1 | ((1 << 1) - 1)) & z_axis,
                        false => !(z_axis << 1) & z_axis,
                    },
                    None => !(z_axis << 1) & z_axis,
                };

                mask.back[i + j * CHUNK_SIZE] = mask_back;

                let mask_front = match front {
                    Some(chunk) => match chunk.get_block(i, j, 0)? != Block::Air {
                        true => !(z_axis >> 1 | (!0 << 31)) & z_axis,
                        false => !(z_axis >> 1) & z_axis,
                    },
                    None => !(z_axis >> 1) & z_axis,
                };

                mask.front[i + j * CHUNK_SIZE] = mask_front;
            }
        }

        Ok(mask)
    }
     */
}
