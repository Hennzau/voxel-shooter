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
            Direction::Back => 4i32,
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

///! plane data with 4 vertices
pub struct Quad {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
    pub direction: Direction,
}

impl Quad {
    // the input position is assumed to be a voxel's (0,0,0) pos
    // therefore right / up / forward are offset by 1
    #[inline]
    pub fn from_direction(
        direction: Direction,
        vertices_offset: usize,
        pos: IVec3,
        color: Color,
    ) -> Self {
        let color = color.to_srgba();

        // Quad, size 1, pos = center
        let (vertices, colors) = match direction {
            Direction::Left => (
                vec![
                    -0.5, -0.5, -0.5, // 0
                    -0.5, -0.5, 0.5, // 1
                    -0.5, 0.5, 0.5, // 2
                    -0.5, 0.5, -0.5, // 3
                ],
                vec![
                    color.red * 0.87,
                    color.green * 0.87,
                    color.blue * 0.87,
                    color.alpha,
                ],
            ),
            Direction::Right => (
                vec![
                    0.5, -0.5, 0.5, // 0
                    0.5, -0.5, -0.5, // 1
                    0.5, 0.5, -0.5, // 2
                    0.5, 0.5, 0.5, // 3
                ],
                vec![
                    color.red * 0.9,
                    color.green * 0.9,
                    color.blue * 0.9,
                    color.alpha,
                ],
            ),
            Direction::Down => (
                vec![
                    -0.5, -0.5, 0.5, // 0
                    0.5, -0.5, 0.5, // 1
                    0.5, -0.5, -0.5, // 2
                    -0.5, -0.5, -0.5, // 3
                ],
                vec![
                    color.red * 0.85,
                    color.green * 0.85,
                    color.blue * 0.85,
                    color.alpha,
                ],
            ),
            Direction::Up => (
                vec![
                    -0.5, 0.5, -0.5, // 0
                    0.5, 0.5, -0.5, // 1
                    0.5, 0.5, 0.5, // 2
                    -0.5, 0.5, 0.5, // 3
                ],
                vec![color.red, color.green, color.blue, color.alpha],
            ),
            Direction::Back => (
                vec![
                    0.5, -0.5, -0.5, // 0
                    -0.5, -0.5, -0.5, // 1
                    -0.5, 0.5, -0.5, // 2
                    0.5, 0.5, -0.5, // 3
                ],
                vec![
                    color.red * 0.88,
                    color.green * 0.88,
                    color.blue * 0.88,
                    color.alpha,
                ],
            ),
            Direction::Forward => (
                vec![
                    -0.5, -0.5, 0.5, // 0
                    0.5, -0.5, 0.5, // 1
                    0.5, 0.5, 0.5, // 2
                    -0.5, 0.5, 0.5, // 3
                ],
                vec![
                    color.red * 0.86,
                    color.green * 0.86,
                    color.blue * 0.86,
                    color.alpha,
                ],
            ),
        };

        Self {
            vertices,
            colors,
            direction,
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
