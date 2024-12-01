// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    //@location(1) color: vec3<f32>,
    //@location(2) vertex_index : u32,
};

@vertex
fn vs_main(
    //@builtin(vertex_index) in_vertex_index: u32,
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    //let x = f32(1 - i32(in_vertex_index)) * 0.5;
    //let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.3;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    //out.color = vec3<f32>(model.tex_coords, 0.5);
    //out.vertex_index = in_vertex_index;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    //let cyan = vec4<f32>(0.0, 1.0, 1.0, 1.0);

    //let vert_color2 = vec4<f32>(0.3, 0.5, in.position);
    //let vert_color = vec4<f32>(in.position, 0.5, 1.0);
    //var grid : vec2<u32>;
    //grid.x = u32(in.position.x / 8.0);
    //grid.y = u32(in.position.y / 8.0);

    //let checker = in.vertex_index % 2u != 1u;
    //let checker = (grid.x + grid.y) % 2u != 1u;
    //return vec4<f32>(in.position, 0.5, 1.0);
    //return select(vert_color2, vert_color, checker);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

