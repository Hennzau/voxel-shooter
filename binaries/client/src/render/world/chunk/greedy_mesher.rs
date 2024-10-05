use bevy::math::{IVec3, UVec3};
use voxel::world::chunk::{Chunk, CHUNK_SIZE};

use crate::render::world::voxel::{Direction, Quad};

pub struct GreedyMesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

fn line(axis: u32, desc: bool, asc: bool) -> (u32, u32) {
    let visible_desc = match desc {
        true => !(axis << 1 | 1) & axis,
        false => !(axis << 1) & axis,
    };

    let visible_asc = match asc {
        true => !(axis >> 1 | 0b01000000000000000000000000000000) & axis,
        false => !(axis >> 1) & axis,
    };

    (visible_desc, visible_asc)
}

fn push_face_in_plane(
    mut axis: u32,
    i: usize,
    j: usize,
    planes: &mut [[u32; CHUNK_SIZE]; CHUNK_SIZE],
) {
    while axis != 0 {
        let count = axis.trailing_zeros() as usize;
        axis &= axis - 1;

        planes[count][i] |= 1 << j;
    }
}

fn push_vertices(
    vertices: &mut Vec<u32>,
    indices: &mut Vec<u32>,
    mut planes: [[u32; CHUNK_SIZE]; CHUNK_SIZE],
    direction: Direction,
) {
    for k in 0..CHUNK_SIZE {
        for i in 0..CHUNK_SIZE {
            let mut j = 0;

            while j < CHUNK_SIZE as u32 {
                j += (planes[k][i] >> j).trailing_zeros();

                if j >= CHUNK_SIZE as u32 {
                    continue;
                }

                let h = (planes[k][i] >> j).trailing_ones();

                let h_as_mask = u32::checked_shl(1, h).map_or(!0, |v| v - 1);
                let mask = h_as_mask << j;

                let mut w = 1;
                while i + w < CHUNK_SIZE {
                    let next_row = (planes[k][i + w] >> j) & h_as_mask;
                    if next_row != h_as_mask {
                        break;
                    }

                    planes[k][i + w] = planes[k][i + w] & !mask;

                    w += 1;
                }

                let pos = match direction {
                    Direction::Left => IVec3::new(k as i32, i as i32, j as i32),
                    Direction::Right => IVec3::new(k as i32, i as i32, j as i32),
                    Direction::Down => IVec3::new(i as i32, k as i32, j as i32),
                    Direction::Up => IVec3::new(i as i32, k as i32, j as i32),
                    Direction::Back => IVec3::new(i as i32, j as i32, k as i32),
                    Direction::Front => IVec3::new(i as i32, j as i32, k as i32),
                };

                let size = match direction {
                    Direction::Left | Direction::Right => UVec3::new(1, w as u32, h as u32),
                    Direction::Down | Direction::Up => UVec3::new(w as u32, 1, h as u32),
                    Direction::Back | Direction::Front => UVec3::new(w as u32, h as u32, 1),
                };

                let Quad {
                    vertices: quad_vertices,
                    indices: quad_indices,
                } = Quad::from_direction(direction, vertices.len(), pos, size);

                vertices.extend(quad_vertices);
                indices.extend(quad_indices);

                j += h;
            }
        }
    }
}

impl GreedyMesh {
    pub fn new(
        chunk: &Chunk,
        left: Option<&Chunk>,
        right: Option<&Chunk>,
        bottom: Option<&Chunk>,
        top: Option<&Chunk>,
        back: Option<&Chunk>,
        front: Option<&Chunk>,
    ) -> eyre::Result<Self> {
        let mut left_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];
        let mut right_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];
        let mut bottom_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];
        let mut top_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];
        let mut back_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];
        let mut front_planes = [[0u32; CHUNK_SIZE]; CHUNK_SIZE];

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x_axis = chunk.x_axis[i + j * CHUNK_SIZE];
                let y_axis = chunk.y_axis[i + j * CHUNK_SIZE];
                let z_axis = chunk.z_axis[i + j * CHUNK_SIZE];

                let left = left
                    .map(|chunk| chunk.x_axis[i + j * CHUNK_SIZE] & (1 << (CHUNK_SIZE - 1)) != 0)
                    .unwrap_or(false);

                let right = right
                    .map(|chunk| chunk.x_axis[i + j * CHUNK_SIZE] & 1 != 0)
                    .unwrap_or(false);

                let bottom = bottom
                    .map(|chunk| chunk.y_axis[i + j * CHUNK_SIZE] & (1 << (CHUNK_SIZE - 1)) != 0)
                    .unwrap_or(false);

                let top = top
                    .map(|chunk| chunk.y_axis[i + j * CHUNK_SIZE] & 1 != 0)
                    .unwrap_or(false);

                let back = back
                    .map(|chunk| chunk.z_axis[i + j * CHUNK_SIZE] & (1 << (CHUNK_SIZE - 1)) != 0)
                    .unwrap_or(false);

                let front = front
                    .map(|chunk| chunk.z_axis[i + j * CHUNK_SIZE] & 1 != 0)
                    .unwrap_or(false);

                // This represent the exact faces that are visible, we now push them in another data structure that contains all the planes that are visible
                let (visible_left, visible_right) = line(x_axis, left, right);
                let (visible_bottom, visible_top) = line(y_axis, bottom, top);
                let (visible_back, visible_front) = line(z_axis, back, front);

                push_face_in_plane(visible_left, i, j, &mut left_planes);
                push_face_in_plane(visible_right, i, j, &mut right_planes);
                push_face_in_plane(visible_bottom, i, j, &mut bottom_planes);
                push_face_in_plane(visible_top, i, j, &mut top_planes);
                push_face_in_plane(visible_back, i, j, &mut back_planes);
                push_face_in_plane(visible_front, i, j, &mut front_planes);
            }
        }

        let mut vertices = vec![];
        let mut indices = vec![];

        push_vertices(&mut vertices, &mut indices, left_planes, Direction::Left);
        push_vertices(&mut vertices, &mut indices, right_planes, Direction::Right);
        push_vertices(&mut vertices, &mut indices, bottom_planes, Direction::Down);
        push_vertices(&mut vertices, &mut indices, top_planes, Direction::Up);
        push_vertices(&mut vertices, &mut indices, back_planes, Direction::Back);
        push_vertices(&mut vertices, &mut indices, front_planes, Direction::Front);

        Ok(Self { vertices, indices })
    }
}
