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

        let encode_x = &data.encode_x;
        let encode_y = &data.encode_y;
        let encode_z = &data.encode_z;

        // Now we are going to make some bitshifts and bitwise operations to know which faces are visible

        for i in 0..16 {
            for j in 0..16 {
                let encoded_x = encode_x[i + j * 16];

                let visible_left = !(encoded_x << 1) & encoded_x;
                let visible_right = !(encoded_x >> 1) & encoded_x;

                // get position of the bits that are set to 1, from
                let visible_left: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_left & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                let visible_right: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_right & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                for k in visible_left {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Left,
                        vertices.len(),
                        bevy::math::IVec3::new(k as i32, i as i32, j as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }

                for k in visible_right {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Right,
                        vertices.len(),
                        bevy::math::IVec3::new(k as i32, i as i32, j as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }

                let encoded_y = encode_y[i + j * 16];

                let visible_bottom = !(encoded_y << 1) & encoded_y;
                let visible_top = !(encoded_y >> 1) & encoded_y;

                let visible_bottom: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_bottom & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                let visible_top: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_top & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                for k in visible_bottom {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Down,
                        vertices.len(),
                        bevy::math::IVec3::new(i as i32, k as i32, j as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }

                for k in visible_top {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Up,
                        vertices.len(),
                        bevy::math::IVec3::new(i as i32, k as i32, j as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }

                let encoded_z = encode_z[i + j * 16];

                let visible_back = !(encoded_z << 1) & encoded_z;
                let visible_forward = !(encoded_z >> 1) & encoded_z;

                let visible_back: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_back & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                let visible_forward: Vec<u8> = (0..16)
                    .filter_map(|i| {
                        if (visible_forward & (1 << i)) != 0 {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                for k in visible_back {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Back,
                        vertices.len(),
                        bevy::math::IVec3::new(i as i32, j as i32, k as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }

                for k in visible_forward {
                    let mut quad = Quad::from_direction(
                        super::quad::Direction::Forward,
                        vertices.len(),
                        bevy::math::IVec3::new(i as i32, j as i32, k as i32),
                        0,
                        15,
                    );

                    vertices.append(&mut quad.vertices);
                    indices.append(&mut quad.indices);
                }
            }
        }

        Self { vertices, indices }
    }
}
