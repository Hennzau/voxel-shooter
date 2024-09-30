use bevy::prelude::*;

// helper
#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
    Back,
    Forward,
}

impl Direction {
    ///! normal data is packed in the shader
    pub fn get_normal(&self) -> i32 {
        match self {
            Direction::Left => 0i32,
            Direction::Right => 1i32,
            Direction::Down => 2i32,
            Direction::Up => 3i32,
            Direction::Back => 5i32,
            Direction::Forward => 5i32,
        }
    }

    pub fn get_opposite(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Back => Direction::Forward,
            Direction::Forward => Direction::Back,
        }
    }
}

///! plane data with 5 vertices
pub struct Quad {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

impl Quad {
    // the input position is assumed to be a voxel's (0,0,0) pos
    // therefore right / up / forward are offset by 1
    #[inline]
    pub fn from_direction(
        direction: Direction,
        vertices_offset: usize,
        pos: IVec3,
        block_id: u32,
        block_health: u32,
    ) -> Self {
        let x = (pos.x as u32).clamp(0, 15);
        let y = (pos.y as u32).clamp(0, 15);
        let z = (pos.z as u32).clamp(0, 15);

        let x1 = (x + 1).clamp(0, 16);
        let y1 = (y + 1).clamp(0, 16);
        let z1 = (z + 1).clamp(0, 16);

        let block_id = block_id.clamp(0, 15);
        let block_health = block_health.clamp(0, 15);

        let vertices = match direction {
            Direction::Left => {
                let normal = 0.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x,
                ]
            }
            Direction::Right => {
                let normal = 1.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x1,
                ]
            }
            Direction::Down => {
                let normal = 2.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x1,
                ]
            }
            Direction::Up => {
                let normal = 3.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x1,
                ]
            }
            Direction::Back => {
                let normal = 5.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y1 << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z << 10 | y << 5 | x1,
                ]
            }
            Direction::Forward => {
                let normal = 5.clamp(0, 6);
                vec![
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y << 5 | x1,
                    normal << 23 | block_health << 19 | block_id << 15 | z1 << 10 | y1 << 5 | x1,
                ]
            }
        };

        Self {
            vertices,
            indices: vec![
                0 + vertices_offset as u32,
                1 + vertices_offset as u32,
                2 + vertices_offset as u32,
                0 + vertices_offset as u32,
                2 + vertices_offset as u32,
                3 + vertices_offset as u32,
            ],
        }
    }
}
