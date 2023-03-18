// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
struct InstanceInput {
    @location(5) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = vec4(model.position + instance.position, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var position = in.tex_coords - vec2(0.5, 0.5);
    var distance_from_middle = length(position);
    var alpha = 1f - step(0.4, distance_from_middle);

    var color = vec3(0.1f, 0.1f, 1.0f); // todo: remove alpha
    color *= smoothstep(0.78, 0.0, distance_from_middle);

    return vec4(color, alpha);
}
