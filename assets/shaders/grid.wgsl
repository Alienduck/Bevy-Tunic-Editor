#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct GridMaterial {
    color: vec4<f32>,
    fade_distance: f32,
    line_width: f32,
    _padding: vec2<f32>,
}

// Auto Batching by Bevy
#ifdef STORAGE_BUFFERS_SUPPORT
@group(2) @binding(0) var<storage, read> material_array: array<GridMaterial>;
#else
@group(2) @binding(0) var<uniform> material_uniform: GridMaterial;
#endif

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    #ifdef STORAGE_BUFFERS_SUPPORT
    let material = material_array[0];
    #else
    let material = material_uniform;
    #endif

    // Coords
    let world_pos = in.world_position.xz;

    // fwidth give the size of a pixel for a perfect anti-aliasing
    let grid_uv = fract(world_pos);
    let fw = fwidth(world_pos);
    let thickness = material.line_width * fw;

    // Smoothing
    let lines = smoothstep(thickness, vec2<f32>(0.0), grid_uv)
              + smoothstep(vec2<f32>(1.0), vec2<f32>(1.0) - thickness, grid_uv);
              
    let alpha = max(lines.x, lines.y);

    // Optimisation : dont compute in a void cube
    if alpha < 0.01 {
        discard;
    }

    // Compute the fade by the distance with the camera
    let dist = distance(in.world_position.xyz, view.world_position);
    let fade = 1.0 - smoothstep(material.fade_distance * 0.3, material.fade_distance, dist);

    return vec4<f32>(material.color.rgb, material.color.a * alpha * fade);
}