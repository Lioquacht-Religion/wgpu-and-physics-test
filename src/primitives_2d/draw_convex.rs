//draw_convex.rs
//

use cgmath::Deg;
use wgpu::util::DeviceExt;

use super::{utils::{Instance2D, Vec2, Mat2}, draw_rect::RectVertex};

pub struct Convex{
    pub vertices : Vec<(f32, f32)>,
    pub color: [f32; 4],
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    instance: Instance2D,
    instance_buffer: wgpu::Buffer,
}


impl Convex{

    pub fn new(device: &wgpu::Device, x: f32, y: f32, vertices : Vec<(f32, f32)>, color: [f32; 4]) -> Self{
        let (vs, is) = Self::get_buffers(device, &vertices, color);
        let instance = Instance2D{
            pos: Vec2::new(x, y),
            rot: Mat2::from_angle(Deg(0.)),
        };
        let instance_data = vec![instance.to_raw()];
        let instance_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&instance_data),
          usage: wgpu::BufferUsages::VERTEX,
         }
        );

        let num_indices = 3*vertices.len() as u32;

        Self {
            vertices,
            color,
            vertex_buffer: vs, index_buffer: is, num_indices,
            instance,
            instance_buffer
        }

    }

    pub fn get_convex_verts_inds(
        init_vertices : &Vec<(f32, f32)>,
        color: [f32; 4]
    ) -> (Vec<RectVertex>, Vec<u16>){
        let color = color;

        let mut vertices = Vec::with_capacity(init_vertices.len()+1);
        let mut indices : Vec<u16> = Vec::with_capacity(3*init_vertices.len());

        vertices.push(RectVertex{position: [0.0, 0.0, 0.0], color});

        let (x, y) = init_vertices[0];
        vertices.push(RectVertex{position: [x, y, 0.0], color});

        for i in 1..init_vertices.len(){
            let (x, y) = init_vertices[i];
            vertices.push(RectVertex{position: [x, y, 0.0], color});
            let ind = i as u16 + 1;
            indices.push(0); indices.push(ind); indices.push(ind-1);
        }

        let ind = (init_vertices.len()) as u16;
        indices.push(0); indices.push(1); indices.push(ind);


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

    pub fn rotate(&mut self, device: &wgpu::Device, rad: f32){
        self.instance.rot = Mat2::from_angle(cgmath::Rad(rad));
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
        vertices : &Vec<(f32, f32)>, color: [f32; 4]
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let (vs, is) = Self::get_convex_verts_inds(vertices, color);
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
