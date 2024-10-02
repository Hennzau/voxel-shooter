#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) data: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) health: f32,
    @location(2) normal: vec3<f32>,
};

var<private> normals: array<vec3<f32>,6> = array<vec3<f32>,6> (
	vec3<f32>(-1.0, 0.0, 0.0), // Left
	vec3<f32>(1.0, 0.0, 0.0), // Right
	vec3<f32>(0.0, -1.0, 0.0), // Down
	vec3<f32>(0.0, 1.0, 0.0), // Up
	vec3<f32>(0.0, 0.0, -1.0), // Forward
	vec3<f32>(0.0, 0.0, 1.0) // Back
);

var<private> colors: array<vec3<f32>,16> = array<vec3<f32>,16>(
    vec3<f32>(0.07, 0.5, 0.07),   // Green (beau vert)
    vec3<f32>(0.0, 0.75, 1.0),  // Sky blue
    vec3<f32>(1.0, 0.5, 0.0),   // Orange
    vec3<f32>(1.0, 0.2, 0.2),   // Coral Red
    vec3<f32>(0.5, 0.0, 0.5),   // Purple
    vec3<f32>(1.0, 1.0, 0.0),   // Yellow
    vec3<f32>(0.0, 0.5, 0.5),   // Teal
    vec3<f32>(0.9, 0.1, 0.8),   // Magenta
    vec3<f32>(0.5, 0.25, 0.0),  // Brown
    vec3<f32>(0.4, 0.8, 0.4),   // Soft Green
    vec3<f32>(0.0, 0.0, 0.75),  // Deep Blue
    vec3<f32>(0.7, 0.7, 0.7),   // Light Grey
    vec3<f32>(0.3, 0.3, 0.3),   // Dark Grey
    vec3<f32>(1.0, 0.6, 0.8),   // Pastel Pink
    vec3<f32>(0.2, 0.8, 0.2),   // Bright Green
    vec3<f32>(1.0, 0.8, 0.6)    // Peach
);

fn x_positive_bits(bits: u32) -> u32{
    return (1u << bits) - 1u;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(vertex.data & x_positive_bits(4u));
    let y = f32(vertex.data >> 4u & x_positive_bits(4u));
    let z = f32(vertex.data >> 8u & x_positive_bits(4u));
    let block_id = u32(vertex.data >> 12u & x_positive_bits(4u));
    let block_health = f32(vertex.data >> 16u & x_positive_bits(4u)) / 15.0;
    let block_normal = normals[u32(vertex.data >> 29u & x_positive_bits(4u))];

    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(x, y, z, 1.0),
    );

    out.color = colors[block_id];
    out.health = block_health;
    out.normal = block_normal;

    return out;
}

struct FragmentInput {
    @location(0) color: vec3<f32>,
    @location(1) health: f32,
    @location(2) normal: vec3<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let color = input.color * input.health;

    let modifier = dot(abs(input.normal), vec3<f32>(0.15, 0.18, 0.12));

    return vec4<f32>(color - modifier, 1.0);
}
