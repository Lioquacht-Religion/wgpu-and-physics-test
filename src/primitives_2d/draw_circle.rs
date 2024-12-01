//draw circle
//
use wgpu::{VertexAttribute, util::DeviceExt, core::instance};

use crate::{model::Vertex, primitives_2d::draw_circle};

use cgmath::{prelude::*, Deg, Rad, Quaternion};

use super::utils::{Instance2D, Instance2DRaw};

type Vec2 = cgmath::Vector2<f32>;

pub struct Circle{
    pub instance: Instance2D,
    pub r: f32,
    pub color: [f32; 4],
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub instance_buffer : wgpu::Buffer,
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleVertex{
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex for CircleVertex{
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<CircleVertex>() as wgpu::BufferAddress,
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


const PRECISION : usize = 18;

impl Circle{

    pub fn new(device: &wgpu::Device, x: f32, y: f32, r: f32, color: [f32; 4]) -> Self{
        let (vs, is) = Self::get_buffers(device, x, y, r, color);
        let instance = Instance2D{
            pos: Vec2::new(x, y),
            rot: cgmath::Matrix2::from_angle(Rad(0.)),
        };
        let instance_buffer = instance.to_raw_buffer(device);
        Self {
            instance,
            r, color,
            vertex_buffer: vs, index_buffer: is, num_indices: draw_circle::PRECISION as u32 *3,
            instance_buffer,
        }

    }

    pub fn get_circle_vertices(
        x: f32, y: f32, r: f32, color: [f32; 4]
    ) -> (Vec<CircleVertex>, Vec<u16>){
        const ANGLE_CHANGE : f32 = 3.14*2. / draw_circle::PRECISION as f32;
        let color = color;

        let mut vertices = Vec::with_capacity(draw_circle::PRECISION+1);
        vertices.push(CircleVertex{position: [x, y, 0.0], color});
        let mut indices : Vec<u16> = Vec::new();

        let mut rot_vert = Vec2::new(0., r);
            vertices.push(
                CircleVertex{
                    position: [x, y+r, 0.0],
                    color
                });

            //println!("angle: 0; x: {}; y: {}", rot_vert.x, rot_vert.y);
        for i in 1..draw_circle::PRECISION{
            //let a : f32 = ANGLE_CHANGE * (draw_circle::PRECISION - i) as f32;
            let a : f32 = ANGLE_CHANGE * i as f32;
            rot_vert.x = a.sin() * r;
            rot_vert.y = a.cos() * r;//( r.powi(2)-rot_vert.x.powi(2) ).sqrt() ;
            //println!("angle: {a}; x: {}; y: {}", rot_vert.x, rot_vert.y);

            vertices.push(
                CircleVertex{
                    position: [x+rot_vert.x, y+rot_vert.y, 0.0],
                    color
                });
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push(i as u16);
        }

            indices.push(0);
            indices.push(1);
            indices.push(draw_circle::PRECISION as u16);

        /*let vertices = vec![

            CircleVertex{position: [x, y, 0.0], color},

            CircleVertex{position: [x, y+r, 0.0], color},
            CircleVertex{position: [x-r, y, 0.0], color},
            CircleVertex{position: [x, y-r, 0.0], color},
            CircleVertex{position: [x+r, y, 0.0], color},
        ];

        let indices = vec![
            0,1,2,
            0,2,3,
            0,3,4,
            0,4,1
        ];*/

        (vertices, indices)
    }


    pub fn translate(&mut self, device: &wgpu::Device, x: f32, y: f32){
        self.instance.pos.x = x;
        self.instance.pos.y = y;

        let instance_data = vec![self.instance.to_raw()];
        let instance_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&instance_data),
          usage: wgpu::BufferUsages::VERTEX,
         }
        );
        self.instance_buffer = instance_buffer;
    }


    pub fn draw<'a, 'b>(
        &'a self,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){
        Self::draw_circle(
            &self.vertex_buffer, &self.index_buffer, self.num_indices,
            &self.instance_buffer,
            render_pipeline, render_pass
        );
    }



    pub fn draw_circle<'a, 'b>(
        vs: &'a wgpu::Buffer,
        is: &'a wgpu::Buffer,
        num_indices: u32,
        instance_buffer: &'a wgpu::Buffer,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){

        render_pass.set_pipeline(
            render_pipeline
        );
        render_pass.set_vertex_buffer(0, vs.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(
            is.slice(..),
            wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..num_indices, 0, 0..1);

    }

    pub fn get_buffers(
        device: &wgpu::Device,
        x: f32, y: f32, r: f32, color: [f32; 4]
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let (vs, is) = Self::get_circle_vertices(x,y,r,color);
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
            wgpu::include_wgsl!("circle.wgsl")
        );
        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Circle Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState{
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        CircleVertex::desc(),
                        Instance2DRaw::desc(),
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
