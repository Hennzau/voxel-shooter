use bevy::prelude::Commands;

use super::{blocks::Block, VoxelWorld};

pub fn generate_tree(commands: &mut Commands, world: &VoxelWorld, x: i32, y: i32, z: i32) {
    let radius = 4;

    for xx in -radius..=radius {
        for yy in -radius..=radius {
            for zz in -radius..=radius {
                if xx * xx + yy * yy + zz * zz <= radius * radius {
                    let yy = yy + 7;

                    world.set_block(commands, x + xx, y + yy, z + zz, Block::Leaves, 15);
                }
            }
        }
    }

    for yy in 0..5 {
        world.set_block(commands, x, y + yy, z, Block::Wood, 15);
    }
}
