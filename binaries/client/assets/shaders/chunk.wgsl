#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) data: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uvw: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

const CHUNK_SIZE: f32 = 31.0;

var<private> colors: array<vec3<f32>,17> = array<vec3<f32>,17>(
    vec3<f32>(0.00, 0.0, 0.00), // Air, should not be rendered
    vec3<f32>(0.07, 0.5, 0.07), // Grass
    vec3<f32>(0.5, 0.25, 0.0),  // Dirt
    vec3<f32>(0.7, 0.7, 0.7),   // Stone
    vec3<f32>(0.07, 0.6, 0.07), // Light Grass
    vec3<f32>(0.35, 0.20, 0.0),  // Woods
    vec3<f32>(0.07, 0.3, 0.07), // Leaves
    vec3<f32>(0.15, 0.6, 0.2), // Light Leaves
    vec3<f32>(0.3, 0.3, 0.3),   // Dark Grey
    vec3<f32>(0.0, 0.75, 1.0),  // Sky blue
    vec3<f32>(1.0, 0.5, 0.0),   // Orange
    vec3<f32>(1.0, 0.2, 0.2),   // Coral Red
    vec3<f32>(0.5, 0.0, 0.5),   // Purple
    vec3<f32>(1.0, 1.0, 0.0),   // Yellow
    vec3<f32>(0.9, 0.1, 0.8),   // Magenta
    vec3<f32>(0.0, 0.0, 0.75),  // Deep Blue
    vec3<f32>(1.0, 0.6, 0.8),   // Pastel Pink
);

fn x_positive_bits(bits: u32) -> u32 {
    return (1u << bits) - 1u;
}

fn interpolate(x: f32) -> f32 {
    return 2 * x - 1;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(vertex.data & x_positive_bits(5u));
    let y = f32(vertex.data >> 5u & x_positive_bits(5u));
    let z = f32(vertex.data >> 10u & x_positive_bits(5u));

    let u_o = interpolate(f32(vertex.data >> 15u & x_positive_bits(1u)));
    let v_o = interpolate(f32(vertex.data >> 16u & x_positive_bits(1u)));
    let w_o = interpolate(f32(vertex.data >> 17u & x_positive_bits(1u)));

    let n_x = f32(vertex.data >> 18u & x_positive_bits(2u)) - 1.0;
    let n_y = f32(vertex.data >> 20u & x_positive_bits(2u)) - 1.0;
    let n_z = f32(vertex.data >> 22u & x_positive_bits(2u)) - 1.0;

    let health = f32(vertex.data >> 24u & x_positive_bits(4u)) / 15.0;

    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(x, y, z, 1.0),
    );

    out.uvw.x = x + u_o * 0.25;
    out.uvw.y = y + v_o * 0.25;
    out.uvw.z = z + w_o * 0.25;

    out.uvw = out.uvw / CHUNK_SIZE;
    out.normal = vec3<f32>(n_x, n_y, n_z);

    return out;
}

@group(2) @binding(0) var chunk: texture_3d<f32>;
@group(2) @binding(1) var chunk_sampler: sampler;

struct FragmentInput {
    @location(0) uvw: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let block = u32(textureSample(chunk, chunk_sampler, input.uvw).x * 255.0);

    let id = block & x_positive_bits(4u);
    let health = f32(block >> 4u) / 15.0;

    let color = colors[id % 16u];

    let modifier = dot(abs(input.normal), vec3<f32>(0.15, 0.18, 0.12));

    return vec4<f32>((color - modifier) * health, 1.0);
}
