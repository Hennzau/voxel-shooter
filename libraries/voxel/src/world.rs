use bevy::{
    prelude::*,
    utils::HashMap,
};
use blocks::Block;
use chunk::{
    Chunk, ChunkModification, ChunkNeighbors, ChunkUpdated, TerrainGenerated, VegetationGenerated,
    CHUNK_SIZE,
};

pub mod blocks;
pub mod chunk;
pub mod tree;

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                load_chunk,
                update_chunk,
                generate_terrain,
                generate_vegetation,
            ),
        );
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

    pub fn set_block(
        &self,
        commands: &mut Commands,
        x: i32,
        y: i32,
        z: i32,
        block: Block,
        health: u8,
    ) {
        let chunk_pos = IVec3::new(
            match x >= 0 {
                true => x / CHUNK_SIZE as i32,
                false => (x - CHUNK_SIZE as i32 + 1) / CHUNK_SIZE as i32,
            },
            match y >= 0 {
                true => y / CHUNK_SIZE as i32,
                false => (y - CHUNK_SIZE as i32 + 1) / CHUNK_SIZE as i32,
            },
            match z >= 0 {
                true => z / CHUNK_SIZE as i32,
                false => (z - CHUNK_SIZE as i32 + 1) / CHUNK_SIZE as i32,
            },
        );

        if let Some(entity) = self.chunks.get(&chunk_pos) {
            commands
                .entity(*entity)
                .add(move |mut entity: EntityWorldMut| {
                    let data = (
                        UVec3::new(
                            x.rem_euclid(CHUNK_SIZE as i32) as u32,
                            y.rem_euclid(CHUNK_SIZE as i32) as u32,
                            z.rem_euclid(CHUNK_SIZE as i32) as u32,
                        ),
                        block,
                        health,
                    );

                    if let Some(mut modification) = entity.get_mut::<ChunkModification>() {
                        modification.blocks.push(data);
                    } else {
                        entity.insert(ChunkModification { blocks: vec![data] });
                    }
                });
        }
    }

    pub fn update_neighbors(&self, commands: &mut Commands, pos: IVec3) {
        let ChunkNeighbors {
            left,
            right,
            front,
            back,
            top,
            bottom,
        } = self.neighbours(pos);

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

fn generate_terrain(
    mut commands: Commands,
    worlds: Query<&VoxelWorld>,
    mut chunks: Query<(Entity, &mut Chunk), Without<TerrainGenerated>>,
) {
    for world in &worlds {
        let mut count = 0;
        for (entity, mut chunk) in &mut chunks {
            if count >= 10 {
                break;
            }
            count += 1;

            let IVec3 { x, y, z } = chunk.pos;

            for xx in 0..CHUNK_SIZE {
                for zz in 0..CHUNK_SIZE {
                    let x = x * CHUNK_SIZE as i32 + xx as i32;
                    let z = z * CHUNK_SIZE as i32 + zz as i32;

                    use perlin2d::PerlinNoise2D;

                    let terrain =
                        PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);
                    let grass_transition =
                        PerlinNoise2D::new(2, 20.0, 20.0, 5.0, 2.0, (100.0, 100.0), 0.5, 188);

                    let height =
                        terrain.get_noise(x as f64, z as f64) as i32 + 20 + CHUNK_SIZE as i32;
                    let grass_level = grass_transition.get_noise(x as f64, z as f64) as i32 + 20;

                    for yy in 0..CHUNK_SIZE {
                        let y = y * CHUNK_SIZE as i32 + yy as i32;

                        if y > height {
                            continue;
                        }

                        let block = if y as i32 >= height - 3 {
                            if y as i32 >= grass_level {
                                Block::LightGrass
                            } else {
                                Block::Grass
                            }
                        } else if y as i32 > height - 15 {
                            Block::Dirt
                        } else {
                            Block::Stone
                        };

                        let random_health = (rand::random::<u8>() % 4) + 12;

                        if let Err(error) = chunk.set_block(xx, yy, zz, block, random_health) {
                            eprintln!("{}", error);
                        }
                    }
                }
            }

            commands.entity(entity).insert(TerrainGenerated);
            commands.entity(entity).insert(ChunkUpdated);
            world.update_neighbors(&mut commands, chunk.pos);
        }
    }
}

fn generate_vegetation(
    mut commands: Commands,
    worlds: Query<&VoxelWorld>,
    chunks: Query<(Entity, &Chunk), Without<VegetationGenerated>>,
) {
    for world in &worlds {
        let mut count = 0;
        for (entity, chunk) in &chunks {
            if count >= 4 {
                break;
            }
            count += 1;

            let IVec3 { x, y, z } = chunk.pos;

            use perlin2d::PerlinNoise2D;

            let terrain = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);

            let trees = (0..5)
                .map(|_| {
                    (
                        rand::random::<u32>() % CHUNK_SIZE as u32 - 1,
                        rand::random::<u32>() % CHUNK_SIZE as u32 - 1,
                    )
                })
                .collect::<Vec<_>>();

            for (tree_x, tree_z) in trees.clone() {
                let x = x * CHUNK_SIZE as i32 + tree_x as i32;
                let z = z * CHUNK_SIZE as i32 + tree_z as i32;

                let height = terrain.get_noise(x as f64, z as f64) as i32 + 20 + CHUNK_SIZE as i32;

                if height >= y * CHUNK_SIZE as i32 + CHUNK_SIZE as i32 {
                    continue;
                }

                if height < y * CHUNK_SIZE as i32 {
                    continue;
                }

                tree::generate_tree(&mut commands, world, x, height, z);
            }

            commands.entity(entity).insert(VegetationGenerated);
        }
    }
}

fn load_chunk(mut commands: Commands, mut worlds: Query<(Entity, &mut VoxelWorld)>) {
    for (entity, mut world) in &mut worlds {
        while let Some(next) = world.next_chunks.pop() {
            let IVec3 { x, y, z } = next;

            if world.chunks.contains_key(&IVec3::new(x, y, z)) {
                continue;
            }

            let chunk = chunk::Chunk::new(IVec3::new(x, y, z));

            commands.entity(entity).with_children(|parent| {
                let id = parent
                    .spawn(chunk)
                    .insert(Name::new(format!("Chunk ({}, {}, {})", x, y, z)))
                    .id();

                world.chunks.insert(IVec3::new(x, y, z), id);
            });
        }
    }
}

fn update_chunk(
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    mut chunks: Query<(Entity, &mut Chunk, &ChunkModification), Without<ChunkUpdated>>,
) {
    for world in &mut worlds {
        for (chunk_id, mut chunk, modification) in &mut chunks {
            if modification.blocks.is_empty() {
                continue;
            }

            for (pos, block, health) in modification.blocks.iter() {
                if let Err(error) = chunk.set_block(
                    pos.x as usize,
                    pos.y as usize,
                    pos.z as usize,
                    *block,
                    *health,
                ) {
                    eprintln!("{}", error);
                }
            }

            commands.entity(chunk_id).remove::<ChunkModification>();

            commands.entity(chunk_id).insert(ChunkUpdated);

            world.update_neighbors(&mut commands, chunk.pos);
        }
    }
}
