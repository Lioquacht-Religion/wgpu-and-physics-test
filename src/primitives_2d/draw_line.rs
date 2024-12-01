//draw_line
//



use wgpu::{VertexAttribute, util::DeviceExt};

use crate::model::Vertex;


pub struct Line{
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub width: f32,
    pub color: [f32; 4],
}

pub struct LineMesh{
    pub points: Vec<[f32; 2]>,
    pub width: f32,
    pub color: [f32; 4],
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineVertex{
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex for LineVertex{
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<LineVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute{
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                VertexAttribute{
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }

    }
}

impl Line {

    pub fn get_line_vertices(&self) -> ([LineVertex; 4], [u16; 6]){
        let color = self.color;
        let (x1, y1, x2, y2) = (self.p1[0], self.p1[1], self.p2[0], self.p2[1]);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let l = dx.hypot(dy);
        let v = dy * self.width * 0.5 / l;
        let u = dx * self.width * 0.5 / l;

        let vertices = [
            LineVertex{position: [x1-v, y1+u, 0.0], color},
            LineVertex{position: [x2-v, y2+u, 0.0], color},
            LineVertex{position: [x2+v, y2-u, 0.0], color},
            LineVertex{position: [x1+v, y1-u, 0.0], color},
        ];

        let indices = [
            //0, 1, 3,
            //1, 2, 3
            3,1,0,
            3,2,1
        ];

        (vertices, indices)
    }

    pub fn draw_line<'a, 'b>
        (
        vs: &'a wgpu::Buffer,
        is: &'a wgpu::Buffer,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    )
    //where 'a : 'b
    {
           render_pass.set_pipeline(
            render_pipeline
        );
        render_pass.set_vertex_buffer(0,
                                      vs.slice(..)
                                      );
        render_pass.set_index_buffer(
            is.slice(..),
            wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);

    }

    pub fn get_buffers(&self, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let (vs, is) = self.get_line_vertices();
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vs),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&is),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        (vertex_buffer, index_buffer)
    }

    pub fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) -> wgpu::RenderPipeline{


        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor{
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],

            }
        );

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("line.wgsl")
        );
        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Line Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState{
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        LineVertex::desc(),
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState{
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })

    }


}

impl LineMesh{

    pub fn new(device: &wgpu::Device, points: Vec<[f32; 2]>, width : f32, color: [f32; 4]) -> Self{

        let (vs, is) = Self::get_buffers(device, &points, width, color);

        let num_indices : u32 = (points.len() as u32)*6 + (points.len() as u32-2)*6;
        //println!("num_indices: {num_indices}");

        Self {
            points,
            width,
            color,
            vertex_buffer: vs,
            index_buffer: is,
            num_indices,
        }
    }

    pub fn get_line_vertices(
        points: &Vec<[f32; 2]>,
        width : f32,
        color: [f32; 4]) -> (Vec<LineVertex>, Vec<u16>){

        let mut vertices : Vec<LineVertex> = vec![];
        let mut indices : Vec<u16> = vec![];
        let color2 : [f32; 4] = [1.0, 0., 0., 1.0];

        if points.len() >= 2 {
        let (mut x1, mut y1) = (points[0][0], points[0][1]);
        let point = points[1];
        let (x2, y2) = (point[0], point[1]);
        let (u, v) = Self::get_u_v(x1, y1, points[1][0], points[1][1], width);

            vertices.push(LineVertex{position: [x1-v, y1+u, 0.0], color});
            vertices.push(LineVertex{position: [x1+v, y1-u, 0.0], color: color2});
            vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

            let t_i = 0;
            indices.push(0+t_i); indices.push(1+t_i); indices.push(2+t_i);
            indices.push(1+t_i); indices.push(3+t_i); indices.push(2+t_i);

        x1 = x2; y1 = y2;


       for i in 1..points.len()-1{
            let point = points[i];
            let (x2, y2) = (point[0], point[1]);
            let (u, v) = Self::get_u_v(x1, y1, x2, y2, width);

            vertices.push(LineVertex{position: [x1-v, y1+u, 0.0], color});
            vertices.push(LineVertex{position: [x1+v, y1-u, 0.0], color: color2});
            vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

            let t_i = 4*(i as u16);
            indices.push(0+t_i); indices.push(1+t_i); indices.push(2+t_i);
            indices.push(1+t_i); indices.push(3+t_i); indices.push(2+t_i);
            indices.push(t_i-2); indices.push(t_i-1); indices.push(t_i);
            indices.push(t_i-2); indices.push(t_i-1); indices.push(t_i+1);
            //indices.push(t_i); indices.push(t_i-1); indices.push(t_i+1);

            x1 = x2; y1 = y2;

        }
            let point = points[points.len()-1];
            let (x2, y2) = (point[0], point[1]);
            let (u, v) = Self::get_u_v(x1, y1, x2, y2, width);

            vertices.push(LineVertex{position: [x1-v, y1+u, 0.0], color});
            vertices.push(LineVertex{position: [x1+v, y1-u, 0.0], color: color2});
            vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

            let t_i = 4*((points.len()-1) as u16);
            indices.push(0+t_i); indices.push(1+t_i); indices.push(2+t_i);
            indices.push(1+t_i); indices.push(3+t_i); indices.push(2+t_i);
            indices.push(t_i-2); indices.push(t_i-1); indices.push(t_i);
            //indices.push(1+t_i); indices.push(t_i-1); indices.push(t_i);

            indices.push(t_i-2); indices.push(t_i-1); indices.push(t_i+1);

        }

        (vertices, indices)
    }



    pub fn get_line_vertices_old(
        points: &Vec<[f32; 2]>,
        width : f32,
        color: [f32; 4]) -> (Vec<LineVertex>, Vec<u16>){

        let mut vertices : Vec<LineVertex> = vec![];
        let mut indices : Vec<u16> = vec![];
        let color2 : [f32; 4] = [1.0, 0., 0., 1.0];

        if points.len() >= 2 {
        let (mut x1, mut y1) = (points[0][0], points[0][1]);
        let point = points[1];
        let (x2, y2) = (point[0], point[1]);
        let (u, v) = Self::get_u_v(x1, y1, points[1][0], points[1][1], width);

            vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

       for i in 1..points.len()-1{
            let point = points[i];
            let (x2, y2) = (point[0], point[1]);
            //let (u, v) = Self::get_u_v(x1, y1, x2, y2, width);

            let corner_point_inner =
                Self::get_line_corner_point(
                    points[i-1], points[i], points[i+1], width, -1.
                );
            let corner_point_outer =
                Self::get_line_corner_point(
                    points[i-1], points[i], points[i+1], width, 1.
                );
            vertices.push(LineVertex{position: [corner_point_outer[0], corner_point_outer[1], 0.0], color});
            vertices.push(LineVertex{position: [corner_point_inner[0], corner_point_inner[1], 0.0], color});

            //vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            //vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

            let t_i = 2*(i as u16 -1);
            indices.push(0+t_i); indices.push(1+t_i); indices.push(2+t_i);
            indices.push(1+t_i); indices.push(3+t_i); indices.push(2+t_i);

            x1 = x2; y1 = y2;

        }
            let point = points[points.len()-1];
            let (x2, y2) = (point[0], point[1]);
            let (u, v) = Self::get_u_v(x1, y1, x2, y2, width);

            vertices.push(LineVertex{position: [x2-v, y2+u, 0.0], color});
            vertices.push(LineVertex{position: [x2+v, y2-u, 0.0], color: color2});

            let t_i = 2*((points.len()-2) as u16);
            indices.push(0+t_i); indices.push(1+t_i); indices.push(2+t_i);
            indices.push(1+t_i); indices.push(3+t_i); indices.push(2+t_i);


        }

        (vertices, indices)
    }
    fn get_u_v(x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> (f32, f32){
            let dx = x2 - x1;
            let dy = y2 - y1;
            let l = dx.hypot(dy);
            let v = dy * width * 0.5 / l;
            let u = dx * width * 0.5 / l;
        (u, v)
    }

    fn get_line_corner_point(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], width : f32, mult: f32) -> [f32; 2]{
        let (u1, v1) = Self::get_u_v(p1[0], p1[1], p2[0], p2[1], width);
        let (uv_x1, uv_y1) = (p1[0]-v1*mult, p1[1]+u1*mult);
        let (u2, v2) = Self::get_u_v(p2[0], p2[1], p3[0], p3[1], width);
        let (uv_x2, uv_y2) = (p3[0]-v2*mult, p3[1]+u2*mult);

        // f(x) = y = m*x + b
        // b = y - m*x
        // m = (y - b)/x
        // y = m*x + (y - m)
        // m = (y2 - y1) / (x2 - x1)

        let mut m1 = (p2[1] - p1[1]) / (p2[0] - p1[0]);
        m1 = if m1.is_normal(){m1}else{0.};
        let b1 = uv_y1 - m1 * uv_x1;
        let mut m2 = (p3[1] - p2[1]) / (p3[0] - p2[0]);
        m2 = if m2.is_normal(){m2}else{0.};
        let b2 = uv_y2 - m2 * uv_x2;
        //calculate intersection of the two lines
        let mut corner_x = (b2 - b1) / (m1 - m2);
        corner_x = if corner_x.is_normal(){corner_x}else{0.};
        let corner_y = m1*corner_x + b1;
        [corner_x, corner_y]
    }


    pub fn update_and_draw<'a, 'b>(
        &'a mut self, device: &'a wgpu::Device, new_points: Vec<[f32; 2]>,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){
        self.update_mesh(device, new_points);
        self.draw(render_pipeline, render_pass);
    }


    pub fn update_mesh(&mut self, device: &wgpu::Device, new_points: Vec<[f32; 2]>){
        self.points = new_points;
        let (vs, is) = Self::get_buffers(device, &self.points, self.width, self.color);
        self.vertex_buffer = vs;
        self.index_buffer = is;
        self.num_indices = (self.points.len() as u32)*6 + (self.points.len() as u32-2)*6;
    }

    pub fn draw<'a, 'b>(
        &'a self,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){
        Self::draw_line(&self.vertex_buffer, &self.index_buffer, self.num_indices, render_pipeline, render_pass)
    }


    pub fn draw_line<'a, 'b>(
        vs: &'a wgpu::Buffer,
        is: &'a wgpu::Buffer,
        num_indices: u32,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    )
    //where 'a : 'b
        {

        //let render_pipeline =
         //   Line::create_render_pipeline(device, config);

        //let (vs, is) = self.get_buffers(device);

        render_pass.set_pipeline(
            render_pipeline
        );
        //render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0,
                                      vs.slice(..)
                                      );
        render_pass.set_index_buffer(
            is.slice(..),
            wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);

    }

    pub fn get_buffers(
        device: &wgpu::Device,
        points: &Vec<[f32; 2]>,
        width : f32,
        color: [f32; 4]
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let (vs, is) = Self::get_line_vertices(points, width, color);
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vs),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&is),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        (vertex_buffer, index_buffer)
    }



}


