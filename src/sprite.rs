//sprite

use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::{texture, model, asset_manager::AssetManager};

pub struct Sprite{
    position: cgmath::Vector3<f32>,
    width: f32,
    height: f32,
    texture_handle: Arc<texture::Texture>,
    pub bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct SpriteVertex{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Sprite{
    pub fn create(
        filepath: &str,
        asset_manager: &mut AssetManager,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self{
        let (width, height) = (0., 0.);
        let texture_handle = asset_manager.add_texture(filepath, device, queue);
        let bind_group = Self::get_bind_group(&texture_handle, device);

        const VERTICES : &[SpriteVertex]= &[
            SpriteVertex{position: [-0.9, 0.9, 0.0], tex_coords: [0.0, 0.0]},
            SpriteVertex{position: [-0.9, -0.9, 0.0], tex_coords: [0.0, 1.0]},
            SpriteVertex{position: [0.9, -0.9, 0.0], tex_coords: [1.0, 1.0]},
            SpriteVertex{position: [0.9, 0.9, 0.0], tex_coords: [1.0, 0.0]},
        ];

        const INDICES: &[u16] = &[
           1, 2, 3,
           0, 1, 3,
        ];

        /*
const VERTICES: &[SpriteVertex] = &[
    SpriteVertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
    SpriteVertex { position: [-0.99513406, 0.96958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
    SpriteVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
    SpriteVertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
    SpriteVertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
*/

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_elements = INDICES.len() as u32;

        Self {
            position: cgmath::Vector3::new(0., 0., 0.),
            width, height,
            texture_handle,
            bind_group,
            vertex_buffer, index_buffer, num_elements
        }
    }

    fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout{
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
                entries: &[
                    wgpu::BindGroupLayoutEntry{
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,

                    },
                    wgpu::BindGroupLayoutEntry{
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },

                ],
                label: Some("texture_bind_group_layout"),
            })
    }

    fn get_bind_group(texture: &texture::Texture, device: &wgpu::Device) -> wgpu::BindGroup{
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &Self::get_bind_group_layout(device),
                entries: &[
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry{
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },

                ],
                label: Some("diffuse_bind_group"),
            }
        )
    }





}

impl model::Vertex for SpriteVertex{

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }



    }

}

