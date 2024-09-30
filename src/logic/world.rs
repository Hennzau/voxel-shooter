use bevy::{prelude::*, utils::HashMap};

pub struct WorldLogic;

#[derive(Debug, Component)]
pub struct WorldData {
    pub chunks: HashMap<IVec3, Entity>,
    pub next_chunks: Vec<IVec3>,
}

impl WorldData {
    pub fn new() -> Self {
        let mut next_chunks = Vec::new();
        for x in -5..=5 {
            for z in -5..=5 {
                next_chunks.push(IVec3::new(x, 0, z));
            }
        }

        Self {
            chunks: HashMap::new(),
            next_chunks,
        }
    }
}

#[derive(Debug, Component)]
pub struct ChunkData {
    // 16x16x16
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub blocks: [i32; 16 * 16 * 16],
}

#[derive(Debug, Component)]
pub struct ChunkUpdated;

impl Plugin for WorldLogic {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_chunks);
    }
}

fn load_chunks(mut commands: Commands, mut worlds: Query<(Entity, &mut WorldData)>) {
    for (entity, mut world) in &mut worlds {
        if let Some(next) = world.next_chunks.pop() {
            let IVec3 { x, y, z } = next;

            if world.chunks.contains_key(&IVec3::new(x, y, z)) {
                return;
            }

            // For example, to generate a Perlin noise 2D terrain:
            use perlin2d::PerlinNoise2D;

            let perlin = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);

            let mut blocks = [0; 16 * 16 * 16];

            for i in 0..16 {
                for j in 0..16 {
                    let height =
                        perlin.get_noise((x * 16 + i) as f64, (z * 16 + j) as f64) as i32 + 16;

                    for k in 0..16 {
                        if k < height {
                            blocks[i as usize + k as usize * 16 + j as usize * 16 * 16] = 1;
                        }
                    }
                }
            }

            commands.entity(entity).with_children(|parent| {
                let id = parent
                    .spawn(ChunkData { x, y, z, blocks })
                    .insert(ChunkUpdated)
                    .insert(Name::new(format!("Chunk ({}, {}, {})", x, y, z)))
                    .id();

                world.chunks.insert(IVec3::new(x, y, z), id);
            });
        }
    }
}
