//line shader



struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) vertex_index : u32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let cyan = vec4<f32>(0.0, 1.0, 1.0, 1.0);

    //let vert_color2 = vec4<f32>(0.3, 0.5, in.position);
    //let vert_color = vec4<f32>(in.position, 0.5, 1.0);
    //var grid : vec2<u32>;
    //grid.x = u32(in.position.x / 8.0);
    //grid.y = u32(in.position.y / 8.0);

    let checker = in.vertex_index % 2u == 1u;
    //let checker = (grid.x + grid.y) % 2u != 1u;
    //return vec4<f32>(in.position, 0.5, 1.0);
    return select(cyan+in.color, red+in.color, checker);

    //return vec4<f32>(in.color);
}
