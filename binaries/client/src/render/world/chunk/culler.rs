use bevy::math::IVec3;
use voxel::world::{
    blocks::Block,
    chunk::{Chunk, CHUNK_SIZE},
};

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
    block: Block,
    health: u32,
) {
    let mut quad = Quad::from_direction(direction, vertices.len(), pos, block as u32 - 1, health);

    vertices.append(&mut quad.vertices);
    indices.append(&mut quad.indices);
}

fn line_axis(axis: u16, before: bool, after: bool) -> (u16, u16) {
    let visible_asc = match after {
        true => !(axis << 1) & axis,
        false => !(axis << 1) & axis,
    };

    let visible_desc = match before {
        true => !(axis >> 1) & axis,
        false => !(axis >> 1) & axis,
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
        let block = chunk.get_block(i, j, k)?;
        let health = chunk.get_health(i, j, k)? as u32;

        push_face(vertices, indices, pos, direction, block, health);
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
        println!("Culling chunk at {:?}", chunk.pos);
        println!("{:?}", back.is_some());
        println!("{:?}", front.is_some());

        let mut vertices = Vec::<u32>::new();
        let mut indices = Vec::<u32>::new();

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x_axis = chunk.x_axis[i + j * CHUNK_SIZE];
                let y_axis = chunk.y_axis[i + j * CHUNK_SIZE];
                let z_axis = chunk.z_axis[i + j * CHUNK_SIZE];

                let (visible_left, visible_right) = line_axis(x_axis, false, false);
                let (visible_bottom, visible_top) = line_axis(y_axis, false, false);
                let (visible_back, visible_front) = line_axis(z_axis, false, false);

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
                            Direction::Forward,
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

        /*
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z)?;
                    let health = chunk.get_health(x, y, z)? as u32;
                    if block == Block::Air {
                        continue;
                    }

                    let pos = IVec3::new(x as i32, y as i32, z as i32);

                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Forward,
                        block,
                        health,
                    );
                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Back,
                        block,
                        health,
                    );
                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Left,
                        block,
                        health,
                    );
                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Right,
                        block,
                        health,
                    );
                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Up,
                        block,
                        health,
                    );
                    push_face(
                        &mut vertices,
                        &mut indices,
                        pos,
                        Direction::Down,
                        block,
                        health,
                    );
                }
            }
        }

         */

        Ok(Self { vertices, indices })
    }
}
