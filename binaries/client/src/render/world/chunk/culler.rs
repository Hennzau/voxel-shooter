use bevy::math::IVec3;
use voxel::world::chunk::{Chunk, CHUNK_SIZE};

use crate::render::world::voxel::{Direction, Quad};

pub struct CulledMesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

fn push_face(
    vertices: &mut Vec<u32>,
    indices: &mut Vec<u32>,
    pos: IVec3,
    direction: Direction,
    health: u32,
) {
    let mut quad = Quad::from_direction(direction, vertices.len(), pos, health);

    vertices.append(&mut quad.vertices);
    indices.append(&mut quad.indices);
}

fn line_axis(axis: u16, before: bool, after: bool) -> (u16, u16) {
    let visible_asc = match after {
        true => !(axis >> 1 | 0b0100000000000000) & axis,
        false => !(axis >> 1) & axis,
    };

    let visible_desc = match before {
        true => !(axis << 1 | 1) & axis,
        false => !(axis << 1) & axis,
    };

    (visible_asc, visible_desc)
}

fn push_face_axis(
    vertices: &mut Vec<u32>,
    indices: &mut Vec<u32>,
    chunk: &Chunk,
    count: u16,
    i: usize,
    j: usize,
    k: usize,
    visible: u16,
    direction: Direction,
) -> eyre::Result<()> {
    if visible & (1 << count) != 0 {
        let pos = IVec3::new(i as i32, j as i32, k as i32);
        let health = chunk.get_health(i, j, k)? as u32;

        push_face(vertices, indices, pos, direction, health);
    }

    Ok(())
}

impl CulledMesh {
    pub fn new(
        chunk: &Chunk,
        left: Option<&Chunk>,
        right: Option<&Chunk>,
        bottom: Option<&Chunk>,
        top: Option<&Chunk>,
        back: Option<&Chunk>,
        front: Option<&Chunk>,
    ) -> eyre::Result<Self> {
        let mut vertices = Vec::<u32>::new();
        let mut indices = Vec::<u32>::new();

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

                let (visible_right, visible_left) = line_axis(x_axis, left, right);
                let (visible_top, visible_bottom) = line_axis(y_axis, bottom, top);
                let (visible_front, visible_back) = line_axis(z_axis, back, front);

                let success: eyre::Result<()> = {
                    for k in 0..CHUNK_SIZE {
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            k,
                            i,
                            j,
                            visible_left,
                            Direction::Left,
                        )?;
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            k,
                            i,
                            j,
                            visible_right,
                            Direction::Right,
                        )?;
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            i,
                            j,
                            k,
                            visible_front,
                            Direction::Front,
                        )?;
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            i,
                            j,
                            k,
                            visible_back,
                            Direction::Back,
                        )?;
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            i,
                            k,
                            j,
                            visible_top,
                            Direction::Up,
                        )?;
                        push_face_axis(
                            &mut vertices,
                            &mut indices,
                            chunk,
                            k as u16,
                            i,
                            k,
                            j,
                            visible_bottom,
                            Direction::Down,
                        )?;
                    }

                    Ok(())
                };

                if let Err(e) = success {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(Self { vertices, indices })
    }
}
