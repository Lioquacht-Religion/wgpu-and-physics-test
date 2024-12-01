//rect shader

struct InstanceInput{
    @location(5) translation: vec3<f32>,
    @location(6) rot_mat_1: vec2<f32>,
    @location(7) rot_mat_2: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat2x2<f32>(
        instance.rot_mat_1,
        instance.rot_mat_2,
    );
    var out: VertexOutput;
    out.color = model.color;
    //out.clip_position = vec4<f32>(model.position, 1.0);
    let rot_vertex = model_matrix * vec2<f32>(model.position.x, model.position.y);
    out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    out.clip_position.x = rot_vertex.x + instance.translation.x;
    out.clip_position.y = rot_vertex.y + instance.translation.y;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
