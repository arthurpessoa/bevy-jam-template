#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct CustomMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: CustomMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

fn random(v: vec3<f32>) -> f32 {
    return fract(sin(dot(v, vec3<f32>(12.9898, 78.233, 45.164))) * 43758.5453);
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    //TODO: Read this from a heightmap, instead.
    var position = vertex.position;
    position.z += random(position) * 10.0 - 1.0; // Random offset between -1.0 and 1.0


    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(position, 1.0),
    );
    out.blend_color = vertex.blend_color;
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return material.color * input.blend_color;
}