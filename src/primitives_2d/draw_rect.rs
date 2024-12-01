//draw_rect.rs
//
//

use cgmath::Deg;
use wgpu::{VertexAttribute, util::DeviceExt};

use crate::model::Vertex;
use crate::primitives_2d::utils::*;

type Vec2 = cgmath::Vector2<f32>;
type Mat2 = cgmath::Matrix2<f32>;

pub struct Rect{
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    instance: Instance2D,
    instance_buffer: wgpu::Buffer,
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectVertex{
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex for RectVertex{
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<RectVertex>() as wgpu::BufferAddress,
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



impl Rect{

    pub fn new(device: &wgpu::Device, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) -> Self{
        let (vs, is) = Self::get_buffers(device, x, y, w, h, color);
        let instance = Instance2D{
            pos: Vec2::new(-0.5, -0.5),
            rot: Mat2::from_angle(Deg(75.)),
        };
        let instance_data = vec![instance.to_raw()];
        let instance_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&instance_data),
          usage: wgpu::BufferUsages::VERTEX,
         }
        );

        Self {
            w, h,
            color,
            vertex_buffer: vs, index_buffer: is, num_indices: 6,
            instance,
            instance_buffer
        }

    }

    pub fn get_rect_verts_inds(
        x: f32, y: f32, w: f32, h: f32, color: [f32; 4]
    ) -> (Vec<RectVertex>, Vec<u16>){
        let color = color;

        let mut vertices = Vec::with_capacity(4);
        let mut indices : Vec<u16> = Vec::with_capacity(6);
        let h_w = w * 0.5;
        let h_h = h * 0.5;
        vertices.push(RectVertex{position: [-h_w, h_h, 0.0], color});
        vertices.push(RectVertex{position: [h_w, h_h, 0.0], color});
        vertices.push(RectVertex{position: [-h_w, -h_h, 0.0], color});
        vertices.push(RectVertex{position: [h_w, -h_h, 0.0], color});
        indices.push(0); indices.push(2); indices.push(1);
        indices.push(2); indices.push(3); indices.push(1);

        (vertices, indices)
    }

    pub fn draw<'a, 'b>(
        &'a self,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){
        Self::draw_rect(
            &self.vertex_buffer, &self.index_buffer,
            &self.instance_buffer, self.num_indices,
            render_pipeline, render_pass
        );
    }

    pub fn draw_rect<'a, 'b>(
        vs: &'a wgpu::Buffer,
        is: &'a wgpu::Buffer,
        instances: &'a wgpu::Buffer,
        num_indices: u32,
        render_pipeline: &'a wgpu::RenderPipeline,
        render_pass: &'b mut wgpu::RenderPass<'a>
    ){

        render_pass.set_pipeline(render_pipeline);
        render_pass.set_vertex_buffer(0, vs.slice(..));
        render_pass.set_vertex_buffer(1, instances.slice(..));
        render_pass.set_index_buffer(
            is.slice(..),
            wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..num_indices, 0, 0..1);

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

    pub fn rotate(&mut self, device: &wgpu::Device, deg: f32){
        self.instance.rot = Mat2::from_angle(Deg(deg));
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

    pub fn get_buffers(
        device: &wgpu::Device,
        x: f32, y: f32, w: f32, h: f32, color: [f32; 4]
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let (vs, is) = Self::get_rect_verts_inds(x,y,w,h,color);
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
            wgpu::include_wgsl!("rect.wgsl")
        );
        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Line Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState{
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        RectVertex::desc(),
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
