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
        if world.chunks.contains_key(&IVec3::new(0, 0, 0)) {
            return;
        }

        let chunk = commands
            .spawn(ChunkData {
                x: 0,
                y: 0,
                z: 0,
                blocks: [1; 16 * 16 * 16],
            })
            .insert(ChunkUpdated)
            .id();

        world.chunks.insert(IVec3::new(0, 0, 0), chunk);
    }
}
