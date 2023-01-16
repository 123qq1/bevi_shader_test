#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {

    let aspect = vec2(16.0,9.0);
    let size = 20.0;

    let scale = aspect * size;

    let uv = coords_to_viewport_uv(position.xy, view.viewport);

    let uv_s = uv * scale;
    let uv_f = floor(uv_s);
    let uv_p = uv_f / scale;

    let color = textureSample(texture, texture_sampler, uv_p);
    return color;
}