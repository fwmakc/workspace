struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) pos: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.world_pos = pos;
    out.center = center;
    out.radius = radius;
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.radius > 0.0 && length(in.world_pos - in.center) > in.radius) {
        discard;
    }
    return in.color;
}
