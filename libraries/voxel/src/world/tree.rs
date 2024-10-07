use bevy::prelude::Commands;

use super::{blocks::Block, VoxelWorld};

pub fn generate_tree(commands: &mut Commands, world: &VoxelWorld, x: i32, y: i32, z: i32) {
    generate_oak(commands, world, x, y, z);
}

pub fn generate_oak(commands: &mut Commands, world: &VoxelWorld, x: i32, y: i32, z: i32) {
    let radius = 4;

    let block = Block::Leaves;

    for xx in -radius..=radius {
        for yy in -radius..=radius {
            for zz in -radius..=radius {
                if xx * xx + yy * yy + zz * zz <= radius * radius {
                    let yy = yy + 7;

                    world.set_block(commands, x + xx, y + yy, z + zz, block, 15);
                }
            }
        }
    }

    for yy in 0..5 {
        world.set_block(commands, x, y + yy, z, Block::Wood, 15);
    }
}

pub fn generate_fir(commands: &mut Commands, world: &VoxelWorld, x: i32, y: i32, z: i32) {
    let radius = 4;

    let block = Block::Leaves;

    for xx in -radius..=radius {
        for yy in -radius..=radius {
            for zz in -radius..=radius {
                if xx * xx + yy * yy + zz * zz <= radius * radius {
                    let yy = yy + 7;

                    world.set_block(commands, x + xx, y + yy, z + zz, block, 15);
                }
            }
        }
    }

    for yy in 0..5 {
        world.set_block(commands, x, y + yy, z, Block::Wood, 15);
    }
}
