#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct Custom2DMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: Custom2DMaterial;
@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    return vec4(1.0,1.0,1.0,1.0);
    //return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}
