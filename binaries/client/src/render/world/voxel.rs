use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;

use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{
    AsBindGroup, Extent3d, PolygonMode, RenderPipelineDescriptor, ShaderRef,
    SpecializedMeshPipelineError, TextureDimension, TextureFormat,
};
use bevy::render::{mesh::MeshVertexAttribute, render_resource::VertexFormat};

use voxel::world::chunk::{Chunk, CHUNK_SIZE};

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
    Back,
    Front,
}

pub struct Quad {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

// x: 5, y: 5, z: 5 -> 15
// u_offset: 1, v_offset: 1, z_offset: 1 -> 3
// x_n: 2, y_n: 2, z_n: 2 -> 6
pub const ATTRIBUTE_VOXEL: MeshVertexAttribute =
    MeshVertexAttribute::new("VoxelVertex", 91010550917, VertexFormat::Uint32);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
    #[texture(0, dimension = "3d")]
    #[sampler(1)]
    pub image_3d: Handle<Image>,
}

impl ChunkMaterial {
    pub fn new(chunk: &Chunk, mut images: &mut ResMut<Assets<Image>>) -> Self {
        let image = Image::new(
            Extent3d {
                width: CHUNK_SIZE as u32,
                height: CHUNK_SIZE as u32,
                depth_or_array_layers: CHUNK_SIZE as u32,
            },
            TextureDimension::D3,
            chunk.blocks_as_u8(),
            TextureFormat::R8Unorm,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );

        Self {
            image_3d: images.add(image),
        }
    }
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/new_chunk.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/new_chunk.wgsl".into()
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
        // descriptor.primitive.polygon_mode = PolygonMode::Line;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

impl Quad {
    pub fn from_direction(direction: Direction, vertices_offset: usize, pos: IVec3) -> Self {
        let x = (pos.x as u32).clamp(0, CHUNK_SIZE as u32 - 1);
        let y = (pos.y as u32).clamp(0, CHUNK_SIZE as u32 - 1);
        let z = (pos.z as u32).clamp(0, CHUNK_SIZE as u32 - 1);

        let x1 = (x + 1).clamp(0, CHUNK_SIZE as u32);
        let y1 = (y + 1).clamp(0, CHUNK_SIZE as u32);
        let z1 = (z + 1).clamp(0, CHUNK_SIZE as u32);

        let vertices = match direction {
            Direction::Left => {
                vec![
                    // n_z | n_y    | n_x    | w_o    | v_o    | u_o   | z      | y      | x
                    1 << 22
                        | 1 << 20
                        | 0 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y << 5
                        | x,
                    1 << 22
                        | 1 << 20
                        | 0 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y << 5
                        | x,
                    1 << 22
                        | 1 << 20
                        | 0 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y1 << 5
                        | x,
                    1 << 22
                        | 1 << 20
                        | 0 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y1 << 5
                        | x,
                ]
            }
            Direction::Right => {
                vec![
                    // n_z2 | n_y   0 | n_x  18 | w_o   7 | v_o   6 | u_o   5| z      | y      | x
                    1 << 22
                        | 1 << 20
                        | 2 << 18
                        | 0 << 17
                        | 0 << 16
                        | 1 << 15
                        | z1 << 10
                        | y << 5
                        | x1,
                    1 << 22
                        | 1 << 20
                        | 2 << 18
                        | 0 << 17
                        | 0 << 16
                        | 1 << 15
                        | z << 10
                        | y << 5
                        | x1,
                    1 << 22
                        | 1 << 20
                        | 2 << 18
                        | 0 << 17
                        | 0 << 16
                        | 1 << 15
                        | z << 10
                        | y1 << 5
                        | x1,
                    1 << 22
                        | 1 << 20
                        | 2 << 18
                        | 0 << 17
                        | 0 << 16
                        | 1 << 15
                        | z1 << 10
                        | y1 << 5
                        | x1,
                ]
            }
            Direction::Down => {
                vec![
                    // n_z2 | n_y   0 | n_x  18 | w_o   7 | v_o   6 | u_o   5| z      | y      | x
                    1 << 22
                        | 0 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y << 5
                        | x,
                    1 << 22
                        | 0 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y << 5
                        | x,
                    1 << 22
                        | 0 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y << 5
                        | x1,
                    1 << 22
                        | 0 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y << 5
                        | x1,
                ]
            }
            Direction::Up => {
                vec![
                    // n_z2 | n_y   0 | n_x  18 | w_o   7 | v_o   6 | u_o   5| z      | y      | x
                    1 << 22
                        | 2 << 20
                        | 1 << 18
                        | 0 << 17
                        | 1 << 16
                        | 0 << 15
                        | z << 10
                        | y1 << 5
                        | x,
                    1 << 22
                        | 2 << 20
                        | 1 << 18
                        | 0 << 17
                        | 1 << 16
                        | 0 << 15
                        | z1 << 10
                        | y1 << 5
                        | x,
                    1 << 22
                        | 2 << 20
                        | 1 << 18
                        | 0 << 17
                        | 1 << 16
                        | 0 << 15
                        | z1 << 10
                        | y1 << 5
                        | x1,
                    1 << 22
                        | 2 << 20
                        | 1 << 18
                        | 0 << 17
                        | 1 << 16
                        | 0 << 15
                        | z << 10
                        | y1 << 5
                        | x1,
                ]
            }
            Direction::Back => {
                vec![
                    // n_z2 | n_y   0 | n_x  18 | w_o   7 | v_o   6 | u_o   5| z      | y      | x
                    0 << 22
                        | 1 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y << 5
                        | x,
                    0 << 22
                        | 1 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y1 << 5
                        | x,
                    0 << 22
                        | 1 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y1 << 5
                        | x1,
                    0 << 22
                        | 1 << 20
                        | 1 << 18
                        | 0 << 17
                        | 0 << 16
                        | 0 << 15
                        | z << 10
                        | y << 5
                        | x1,
                ]
            }
            Direction::Front => {
                vec![
                    // n_z2 | n_y   0 | n_x  18 | w_o   7 | v_o   6 | u_o   5| z      | y      | x
                    2 << 22
                        | 1 << 20
                        | 1 << 18
                        | 1 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y1 << 5
                        | x,
                    2 << 22
                        | 1 << 20
                        | 1 << 18
                        | 1 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y << 5
                        | x,
                    2 << 22
                        | 1 << 20
                        | 1 << 18
                        | 1 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y << 5
                        | x1,
                    2 << 22
                        | 1 << 20
                        | 1 << 18
                        | 1 << 17
                        | 0 << 16
                        | 0 << 15
                        | z1 << 10
                        | y1 << 5
                        | x1,
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
