use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;

use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::render::{mesh::MeshVertexAttribute, render_resource::VertexFormat};

use voxel::world::chunk::CHUNK_SIZE;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
    Back,
    Front,
}

/// Each vertex is a u32 with the following format:
/// 0-3: x
/// 4-7: y
/// 8-11: z
/// 12-15: block_id
/// 16-19: block_health
/// 20-23: normal
pub struct Quad {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

pub const ATTRIBUTE_VOXEL: MeshVertexAttribute =
    MeshVertexAttribute::new("VoxelVertex", 988540917, VertexFormat::Uint32); // x: 5, y: 5, z: 5, block_id: 4, block_health: 4, normal: 4,

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/chunk.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/chunk.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[ATTRIBUTE_VOXEL.at_shader_location(0)])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

impl Quad {
    pub fn from_direction(
        direction: Direction,
        vertices_offset: usize,
        pos: IVec3,
        block_id: u32,
        block_health: u32,
    ) -> Self {
        let x = (pos.x as u32).clamp(0, CHUNK_SIZE as u32 - 1);
        let y = (pos.y as u32).clamp(0, CHUNK_SIZE as u32 - 1);
        let z = (pos.z as u32).clamp(0, CHUNK_SIZE as u32 - 1);

        let x1 = (x + 1).clamp(0, CHUNK_SIZE as u32);
        let y1 = (y + 1).clamp(0, CHUNK_SIZE as u32);
        let z1 = (z + 1).clamp(0, CHUNK_SIZE as u32);

        let block_id = block_id.clamp(0, 15);
        let block_health = block_health.clamp(0, 15);

        let vertices = match direction {
            Direction::Left => {
                let normal = 0.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x,
                ]
            }
            Direction::Right => {
                let normal = 1.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x1,
                ]
            }
            Direction::Down => {
                let normal = 2.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x1,
                ]
            }
            Direction::Up => {
                let normal = 3.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x1,
                ]
            }
            Direction::Back => {
                let normal = 5.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y1 << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z << 8 | y << 4 | x1,
                ]
            }
            Direction::Front => {
                let normal = 5.clamp(0, 6);
                vec![
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y << 4 | x1,
                    normal << 20 | block_health << 16 | block_id << 12 | z1 << 8 | y1 << 4 | x1,
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
