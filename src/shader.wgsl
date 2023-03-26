// Vertex shader

struct CameraUniform {
    view_projection: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_projection * vec4(model.position + instance.position, 1.0); 
    out.color = instance.color;

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var position = in.tex_coords - vec2(0.5, 0.5);
    var distance_from_middle = length(position);
    var alpha = 1f - step(0.4, distance_from_middle);

    var color = vec3(0.1f, 0.1f, 1.0f); 
    /* color = in.color; */
    color *= smoothstep(0.5, 0.0, distance_from_middle);

    return vec4(color, alpha);
}
