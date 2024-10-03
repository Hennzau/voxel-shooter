use bevy::{prelude::*, utils::HashMap};
use blocks::Block;
use chunk::{ChunkNeighbors, ChunkUpdated, CHUNK_SIZE};

pub mod blocks;
pub mod chunk;

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_chunk);
    }
}

#[derive(Debug, Component)]
pub struct VoxelWorld {
    pub chunks: HashMap<IVec3, Entity>,
    pub next_chunks: Vec<IVec3>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            next_chunks: Vec::new(),
        }
    }

    pub fn with_generation(mut self, chunks: Vec<IVec3>) -> Self {
        self.generate(chunks);

        self
    }

    pub fn generate(&mut self, chunks: Vec<IVec3>) {
        self.next_chunks.extend(chunks);
    }

    pub fn neighbours(&self, pos: IVec3) -> ChunkNeighbors {
        let IVec3 { x, y, z } = pos;

        ChunkNeighbors {
            left: self.chunks.get(&IVec3::new(x - 1, y, z)).cloned(),
            right: self.chunks.get(&IVec3::new(x + 1, y, z)).cloned(),
            front: self.chunks.get(&IVec3::new(x, y, z + 1)).cloned(),
            back: self.chunks.get(&IVec3::new(x, y, z - 1)).cloned(),
            top: self.chunks.get(&IVec3::new(x, y + 1, z)).cloned(),
            bottom: self.chunks.get(&IVec3::new(x, y - 1, z)).cloned(),
        }
    }
}

fn load_chunk(mut commands: Commands, mut worlds: Query<(Entity, &mut VoxelWorld)>) {
    for (entity, mut world) in &mut worlds {
        if let Some(next) = world.next_chunks.pop() {
            let IVec3 { x, y, z } = next;

            if world.chunks.contains_key(&IVec3::new(x, y, z)) {
                continue;
            }

            use perlin2d::PerlinNoise2D;

            let perlin = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);

            let mut chunk = chunk::Chunk::new(IVec3::new(x, y, z));

            for xx in 0..CHUNK_SIZE {
                for zz in 0..CHUNK_SIZE {
                    let height = perlin
                        .get_noise((x * 16 + xx as i32) as f64, (z * 16 + zz as i32) as f64)
                        as i32
                        + 18;

                    for yy in 0..CHUNK_SIZE {
                        if yy as i32 >= height {
                            continue;
                        }

                        let block = if yy as i32 >= height - 2 {
                            Block::Grass
                        } else if yy as i32 > height - 5 {
                            Block::Dirt
                        } else {
                            Block::Stone
                        };

                        if let Err(error) = chunk.set_block(xx, yy as usize, zz, block, 15) {
                            eprintln!("{}", error);
                        }
                    }
                }
            }

            commands.entity(entity).with_children(|parent| {
                let id = parent
                    .spawn(chunk)
                    .insert(Name::new(format!("Chunk ({}, {}, {})", x, y, z)))
                    .id();

                world.chunks.insert(IVec3::new(x, y, z), id);
            });

            let ChunkNeighbors {
                left,
                right,
                front,
                back,
                top,
                bottom,
            } = world.neighbours(IVec3::new(x, y, z));

            if let Some(left) = left {
                commands.entity(left).insert(ChunkUpdated);
            }

            if let Some(right) = right {
                commands.entity(right).insert(ChunkUpdated);
            }

            if let Some(front) = front {
                commands.entity(front).insert(ChunkUpdated);
            }

            if let Some(back) = back {
                commands.entity(back).insert(ChunkUpdated);
            }

            if let Some(top) = top {
                commands.entity(top).insert(ChunkUpdated);
            }

            if let Some(bottom) = bottom {
                commands.entity(bottom).insert(ChunkUpdated);
            }
        }
    }
}
