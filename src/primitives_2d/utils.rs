
//Instancing to apply changes to 2D primitive such as position and rotation

use wgpu::util::DeviceExt;

pub type Vec2 = cgmath::Vector2<f32>;
pub type Mat2 = cgmath::Matrix2<f32>;
pub type Radians = cgmath::Rad<f32>;

pub struct Instance2D{
    pub pos: Vec2,
    pub rot: Mat2,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance2DRaw {
    pub pos: [f32; 3],
    pub rot: [[f32; 2]; 2],
}

impl Instance2D{
    pub fn to_raw(&self) -> Instance2DRaw{
        Instance2DRaw {
            pos: [self.pos.x, self.pos.y, 0.],
            rot: self.rot.into(),
        }
    }

    pub fn to_raw_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer{
        let instance_data = vec![self.to_raw()];
        let instance_buffer = device.create_buffer_init(
         &wgpu::util::BufferInitDescriptor {
          label: Some("Instance Buffer"),
          contents: bytemuck::cast_slice(&instance_data),
          usage: wgpu::BufferUsages::VERTEX,
         }
        );
        instance_buffer

    }
}

impl Instance2DRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Instance2DRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x2,
                },
           ],
        }
    }
}

