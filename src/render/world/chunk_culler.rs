use super::Quad;
use crate::logic::world::ChunkData;

pub struct ChunkMesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

impl ChunkMesh {
    pub fn from_chunk_data(data: &ChunkData) -> Self {
        let mut vertices = Vec::<u32>::new();
        let mut indices = Vec::<u32>::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if data.blocks[x + y * 16 + z * 16 * 16] == 0 {
                        continue;
                    }

                    let (x, y, z) = (x as i32, y as i32, z as i32);

                    let mut forward = Quad::from_direction(
                        super::quad::Direction::Forward,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );

                    vertices.append(&mut forward.vertices);
                    indices.append(&mut forward.indices);

                    let mut back = Quad::from_direction(
                        super::quad::Direction::Back,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );

                    vertices.append(&mut back.vertices);
                    indices.append(&mut back.indices);

                    let mut left = Quad::from_direction(
                        super::quad::Direction::Left,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );
                    vertices.append(&mut left.vertices);
                    indices.append(&mut left.indices);

                    let mut right = Quad::from_direction(
                        super::quad::Direction::Right,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );

                    vertices.append(&mut right.vertices);
                    indices.append(&mut right.indices);

                    let mut top = Quad::from_direction(
                        super::quad::Direction::Up,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );

                    vertices.append(&mut top.vertices);
                    indices.append(&mut top.indices);

                    let mut bottom = Quad::from_direction(
                        super::quad::Direction::Down,
                        vertices.len(),
                        bevy::math::IVec3::new(x, y, z),
                        0,
                        15,
                    );

                    vertices.append(&mut bottom.vertices);
                    indices.append(&mut bottom.indices);
                }
            }
        }

        Self { vertices, indices }
    }
}
