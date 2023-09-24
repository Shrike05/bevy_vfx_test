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
    //return vec4(1.0,1.0,1.0,1.0);
    return vec4<f32>(
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(0.01, -0.01)).r,
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(-0.01, 0.0)).g,
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(0.0, 0.01)).b,
        1.0
    );
    //return material.color * textureSample(color_texture, color_sampler, mesh.uv);
}
