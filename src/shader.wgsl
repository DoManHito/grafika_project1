struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct ModelUniform {
    model_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> model: ModelUniform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color_front: vec4<f32>,
    @location(2) color_back: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color_front: vec4<f32>,
    @location(1) color_back: vec4<f32>,
};

@vertex
fn vs_main(
    input: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * model.model_matrix * input.position;
    out.color_front = input.color_front;
    out.color_back = input.color_back;
    return out;
}

@fragment
fn fs_main(
    in: VertexOutput, 
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    if (is_front) {
        return in.color_front;
    } else {
        return in.color_back;
    }
}
