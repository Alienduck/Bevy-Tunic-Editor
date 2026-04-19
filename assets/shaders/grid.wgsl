#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<storage, read> settings: array<vec4<f32>, 2>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = settings[0];
    let fade_distance = settings[1].x;
    let line_width = settings[1].y;

    let world_pos = in.world_position.xz;
    let grid_uv = fract(world_pos);
    let fw = fwidth(world_pos);
    let thickness = line_width * fw;

    let edge_dist = min(grid_uv, vec2<f32>(1.0) - grid_uv);
    let lines = 1.0 - smoothstep(thickness * 0.5, thickness, edge_dist);
    let alpha = max(lines.x, lines.y);

    if alpha < 0.01 {
        discard;
    }

    let dist = distance(in.world_position.xyz, view.world_position);
    let fade = 1.0 - smoothstep(fade_distance * 0.3, fade_distance, dist);

    return vec4<f32>(color.rgb, color.a * alpha * fade);
}