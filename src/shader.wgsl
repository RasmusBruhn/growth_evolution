// Uniforms
// The draw mode, 0: fill mode, 1: edge mode
@group(0) @binding(0)
var<uniform> draw_mode: u32;

// The color to use for drawing edges
@group(0) @binding(1)
var<uniform> edge_color: vec4<f32>;

// Type definitions
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    @location(0) hex_offset: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    var pos = hex_offset;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    if draw_mode == 0u {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    } else {
        return edge_color;
    }
}