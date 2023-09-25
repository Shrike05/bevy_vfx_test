#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct Custom2DMaterial {
    color: vec4<f32>,
    intensity: f32,
    vignette: f32,
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
    var i = material.intensity;

    var x = mesh.uv.x - 0.5;
    var y = mesh.uv.y - 0.5;
  
    var a = 1.- material.vignette*sqrt(x*x + y*y);

    return material.color * vec4<f32>(
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(i, -i)).r,
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(-i, 0.0)).g,
        textureSample(color_texture, color_sampler, mesh.uv + vec2<f32>(0.0, i)).b,
        1.0
    ) * vec4<f32>(a,a,a,1.);
}
