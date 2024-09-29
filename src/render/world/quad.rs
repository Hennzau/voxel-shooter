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
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
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

        // Quad, size 1, not centered around pos
        let (vertices, colors) = match direction {
            Direction::Left => (
                vec![
                    [pos.x as f32, pos.y as f32, pos.z as f32], // bottom-left
                    [pos.x as f32, pos.y as f32, pos.z as f32 + 1.0], // bottom-right
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // top-right
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32], // top-left
                ],
                vec![
                    [
                        color.red * 0.85,
                        color.green * 0.85,
                        color.blue * 0.85,
                        color.alpha,
                    ],
                    [
                        color.red * 0.85,
                        color.green * 0.85,
                        color.blue * 0.85,
                        color.alpha,
                    ],
                    [
                        color.red * 0.85,
                        color.green * 0.85,
                        color.blue * 0.85,
                        color.alpha,
                    ],
                    [
                        color.red * 0.85,
                        color.green * 0.85,
                        color.blue * 0.85,
                        color.alpha,
                    ],
                ],
            ),
            Direction::Right => (
                vec![
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32 + 1.0], // bottom-right
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32],       // bottom-left
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32], // top-left
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // top-right
                ],
                vec![
                    [
                        color.red * 0.9,
                        color.green * 0.9,
                        color.blue * 0.9,
                        color.alpha,
                    ],
                    [
                        color.red * 0.9,
                        color.green * 0.9,
                        color.blue * 0.9,
                        color.alpha,
                    ],
                    [
                        color.red * 0.9,
                        color.green * 0.9,
                        color.blue * 0.9,
                        color.alpha,
                    ],
                    [
                        color.red * 0.9,
                        color.green * 0.9,
                        color.blue * 0.9,
                        color.alpha,
                    ],
                ],
            ),
            Direction::Down => (
                vec![
                    [pos.x as f32, pos.y as f32, pos.z as f32], // bottom-left
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32], // bottom-right
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32 + 1.0], // top-right
                    [pos.x as f32, pos.y as f32, pos.z as f32 + 1.0], // top-left
                ],
                vec![
                    [
                        color.red * 0.7,
                        color.green * 0.7,
                        color.blue * 0.7,
                        color.alpha,
                    ],
                    [
                        color.red * 0.7,
                        color.green * 0.7,
                        color.blue * 0.7,
                        color.alpha,
                    ],
                    [
                        color.red * 0.7,
                        color.green * 0.7,
                        color.blue * 0.7,
                        color.alpha,
                    ],
                    [
                        color.red * 0.7,
                        color.green * 0.7,
                        color.blue * 0.7,
                        color.alpha,
                    ],
                ],
            ),
            Direction::Up => (
                vec![
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // bottom-left
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // bottom-right
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32], // top-right
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32],       // top-left
                ],
                vec![
                    [color.red, color.green, color.blue, color.alpha],
                    [color.red, color.green, color.blue, color.alpha],
                    [color.red, color.green, color.blue, color.alpha],
                    [color.red, color.green, color.blue, color.alpha],
                ],
            ),
            Direction::Back => (
                vec![
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32], // bottom-right
                    [pos.x as f32, pos.y as f32, pos.z as f32],       // bottom-left
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32], // top-left
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32], // top-right
                ],
                vec![
                    [
                        color.red * 0.6,
                        color.green * 0.6,
                        color.blue * 0.6,
                        color.alpha,
                    ],
                    [
                        color.red * 0.6,
                        color.green * 0.6,
                        color.blue * 0.6,
                        color.alpha,
                    ],
                    [
                        color.red * 0.6,
                        color.green * 0.6,
                        color.blue * 0.6,
                        color.alpha,
                    ],
                    [
                        color.red * 0.6,
                        color.green * 0.6,
                        color.blue * 0.6,
                        color.alpha,
                    ],
                ],
            ),
            Direction::Forward => (
                vec![
                    [pos.x as f32, pos.y as f32, pos.z as f32 + 1.0], // bottom-left
                    [pos.x as f32 + 1.0, pos.y as f32, pos.z as f32 + 1.0], // bottom-right
                    [pos.x as f32 + 1.0, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // top-right
                    [pos.x as f32, pos.y as f32 + 1.0, pos.z as f32 + 1.0], // top-left
                ],
                vec![
                    [
                        color.red * 0.8,
                        color.green * 0.8,
                        color.blue * 0.8,
                        color.alpha,
                    ],
                    [
                        color.red * 0.8,
                        color.green * 0.8,
                        color.blue * 0.8,
                        color.alpha,
                    ],
                    [
                        color.red * 0.8,
                        color.green * 0.8,
                        color.blue * 0.8,
                        color.alpha,
                    ],
                    [
                        color.red * 0.8,
                        color.green * 0.8,
                        color.blue * 0.8,
                        color.alpha,
                    ],
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
