use bevy::{prelude::*, utils::HashMap};

pub struct WorldLogic;

#[derive(Debug, Component)]
pub struct WorldData {
    pub chunks: HashMap<IVec3, Entity>,
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

fn load_chunks(mut commands: Commands, mut worlds: Query<&mut WorldData>) {
    for mut world in &mut worlds {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-5..=5);
        let y = 0;
        let z = rng.gen_range(-5..=5);

        if world.chunks.contains_key(&IVec3::new(x, y, z)) {
            return;
        }

        // For example, to generate a Perlin noise 2D terrain:
        use perlin2d::PerlinNoise2D;

        let perlin = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 101);

        let mut blocks = [0; 16 * 16 * 16];

        for i in 0..16 {
            for j in 0..16 {
                let height = perlin.get_noise((x * 16 + i) as f64, (z * 16 + j) as f64) as i32 + 16;

                for k in 0..16 {
                    if k < height {
                        blocks[i as usize + k as usize * 16 + j as usize * 16 * 16] = 1;
                    }
                }
            }
        }

        let chunk = commands
            .spawn(ChunkData { x, y, z, blocks })
            .insert(ChunkUpdated)
            .id();

        world.chunks.insert(IVec3::new(x, y, z), chunk);
    }
}
