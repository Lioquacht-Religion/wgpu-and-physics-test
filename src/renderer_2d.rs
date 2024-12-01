//
//


use cgmath::InnerSpace;
use wgpu::{util::{DeviceExt, RenderEncoder}, core::device};
use winit::{window::Window, event::{WindowEvent, MouseButton, ElementState}};

use crate::{texture, asset_manager::AssetManager, sprite::{self, Sprite},
    primitives_2d::{draw_line::{Line, self, LineMesh}, draw_circle::{self, Circle}, draw_rect::Rect, draw_convex::Convex},
    physics_engine::{physic_obj_traits::{CollisionType, CollisionRelation, NodeObject}, chain_body, self, physics_world::{self, World}, convex_body::{Convex2D, self}, col_relations::*, circle_body}};

pub struct Renderer2D{
    pub state : State,

}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}


const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
];


const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];



impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

pub struct State{
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size : winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group : wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    assets : AssetManager,
    line: draw_line::Line,
    line_past: draw_line::Line,
    line_mesh : draw_line::LineMesh,
    chain: chain_body::Chain,
    physics_engine: physics_world::World,
    circles: Vec<draw_circle::Circle>,
    circle_rp : wgpu::RenderPipeline,
    rect: Convex,
    pub window : Window,
}

impl Renderer2D {
    pub async fn new(window : Window) -> Self{
        Self{ state: State::new(window).await}
    }
}

impl State{
    pub async fn new(window: Window) -> Self{
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied().filter(|f| f.is_srgb())
            .next().unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let mut assets = AssetManager::new();
        let airthrow_texture = assets.add_texture(
                "res/airthrow/FH-airThrow0.png",
                &device, &queue,
        );
        let tree_texture = assets.add_texture(
                "res/happy-tree.png",
                &device, &queue,
        );




        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        let texture_bind_group_layout =
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
            });

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry{
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },

                ],
                label: Some("diffuse_bind_group"),
            }
        );




        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor{
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],

            }
            );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState{
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex::desc(),
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
            });

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

        let num_indices = INDICES.len() as u32;

        let line = Line{
            p1: [-0.5, 0.5], p2: [0.5, -0.5], width: 0.2, color: [1.0, 1.0, 0.0, 1.0]
        };

        let line_past = Line{
            p1: [-0.5, 0.5], p2: [0.5, -0.5], width: 0.8, color: [1.0, 1.0, 0.0, 1.0]
        };

        let mesh_coords =
            vec![[-0.9, -0.9], [-0.6, -0.9], [-0.6, -0.6],
            [-0.3, -0.6], [-0.3, -0.3], [0.0, -0.3],
            [0.0, 0.0], [0.3, 0.0], [0.3, 0.3]
            ];

        let line_mesh = draw_line::LineMesh::new(
            &device,
            mesh_coords.clone(),
            0.05,
            [0., 1., 1., 1.]
            );

        let chain = chain_body::Chain::from_coords(
            mesh_coords,
            1., 0.3, 0.002
        );

        let mut physics_engine = physics_world::World::new();
        physics_engine.add_circles();
        let circles = physics_engine.physics_objects.iter_mut().filter_map(
            |c| {
                if let CollisionType::Circle(p_c) = c.get_col_type(){
                    Some(draw_circle::Circle::new(&device, 0., 0., p_c.r, [1.0, 0., 0., 1.]))
                }else{None}
            }
        ).collect();


        let circle_rp = Circle::create_render_pipeline(&device, &config);

        //let rect = Rect::new(&device, 0.5, 0.5, 0.2, 0.3, [0.1, 0.0, 0.0, 1.0]);

        let mut convex2d = convex_body::Convex2D::new(0.3, 0.3,
                        vec![(-0.2, 0.3), (0.2, 0.3), (0.35, 0.0), (0.2, -0.3), (-0.2, -0.3), (-0.35, 0.0)],
                       1.0);
        //convex2d.static_body = true;
        let mut convex2d2 = convex_body::Convex2D::new(-0.5, -0.5,
                        vec![(-0.2, 0.2), (0.2, 0.2), (0.35, 0.0), (0.2, -0.2), (-0.2, -0.2), (-0.35, 0.0)],
                       1.0);


        let mut wall1 = convex_body::Convex2D::new(-1.0, 0.0,
                        vec![(-0.2, 3.0), (0.1, 3.0), (0.1, -3.0), (-0.2, -3.0)],
                       f32::MAX);
        wall1.static_body = true;

        let mut wall2 = convex_body::Convex2D::new(0.0, 1.0,
                        vec![(-3.0, 0.2), (3.0, 0.2), (3.0, -0.1), (-3.0, -0.1)],
                       f32::MAX);
        wall2.static_body = true;

        let mut wall3 = convex_body::Convex2D::new(1.0, 0.0,
                        vec![(-0.1, 3.0), (0.2, 3.0), (0.2, -3.0), (-0.1, -3.0)],
                       f32::MAX);
        wall3.static_body = true;

        let mut wall4 = convex_body::Convex2D::new(0.0, -1.0,
                        vec![(-3.0, 0.1), (3.0, 0.1), (3.0, -0.2), (-3.0, -0.2)],
                       f32::MAX);
        wall4.static_body = true;

        physics_engine.physics_objects.insert(Box::new(wall1));
        physics_engine.physics_objects.insert(Box::new(wall2));
        physics_engine.physics_objects.insert(Box::new(wall3));
        physics_engine.physics_objects.insert(Box::new(wall4));

        physics_engine.physics_objects.insert(Box::new(convex2d2));
        physics_engine.physics_objects.insert(Box::new(convex2d));

        let rect = Convex::new(&device, 0.2, 0.2,
                        vec![(-0.2, 0.3), (0.2, 0.3), (0.35, 0.0), (0.2, -0.3), (-0.2, -0.3), (-0.35, 0.0)],
                               [0.1, 0.0, 0.0, 1.0]);


        Self{
            surface, size, config, device, queue, window,
            render_pipeline, vertex_buffer, index_buffer,num_indices,
            diffuse_bind_group, diffuse_texture, assets,
            line, line_past, line_mesh, chain,
            physics_engine, circles, rect, circle_rp,
        }
    }


    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            //self.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            //self.depth_texture =
                //texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved { device_id, position, modifiers }
            => {
                self.line_past.p1[0] = self.line_past.p2[0];
                self.line_past.p1[1] = self.line_past.p2[1];
                self.line_past.p2[0] = (position.x as f32 - (self.size.width as f32 * 0.5)) / (self.size.width as f32 * 0.5) ;
                self.line_past.p2[1] = -(position.y as f32 - (self.size.height as f32 * 0.5)) / (self.size.height as f32 * 0.5) ;
            },
            WindowEvent::MouseInput { device_id, state, button, modifiers }
            => {
                match state {
                    ElementState::Released => {
                       self.line.p1[0] = self.line.p2[0];
                       self.line.p1[1] = self.line.p2[1];
                       self.line.p2[0] = self.line_past.p2[0];
                       self.line.p2[1] = self.line_past.p2[1];

                    },
                    _=>{}
                }
                match button {
                    MouseButton::Right => {},
                    _=>{}
                }
            },
            _ => {},
        }
        //self.camera_controller.process_events(event)
        false
    }

    pub fn update(&mut self) {
        //self.camera_controller.update_camera(&mut self.camera);
        //self.camera_uniform.update_view_proj(&self.camera);
        /*self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );*/
    }


    const delta_time : std::time::Duration = std::time::Duration::new(0, 1_000_000_000/60);

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        let last_systemtime = std::time::SystemTime::now();

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor{
                label: Some("Render Encoder"),
            });

        let sprite = Sprite::create(

                "res/airthrow/FH-airThrow0.png",
                //"res/happy-tree.png",
                &mut self.assets, &self.device, &self.queue
        );
/*
        let line = Line{
            p1: [-0.5, 0.5], p2: [0.5, -0.5], width: 0.8, color: [1.0, 1.0, 0.0, 1.0]
        };
*/
        self.chain.gravity_dir =
            cgmath::Vector2::new(self.line.p2[0], self.line.p2[1]).normalize();
        self.chain.simulation_step();
        for i in 0..self.chain.points.len(){
            self.line_mesh.points[i][0] = self.chain.points[i].x;
            self.line_mesh.points[i][1] = self.chain.points[i].y;
        }
        let (vs2, is2) = LineMesh::get_buffers(
            &self.device, &self.line_mesh.points,
            self.line_mesh.width, self.line_mesh.color
        );
        self.line_mesh.vertex_buffer = vs2;
        self.line_mesh.index_buffer = is2;

        let (vs, is) = self.line.get_buffers(&self.device);
        let line_rp = Line::create_render_pipeline(&self.device, &self.config);
        //let (vs2, is2) = LineMesh::
        self.physics_engine.global_gravity_dir =
            cgmath::Vector2::new(self.line.p2[0], self.line.p2[1]).normalize();
        self.physics_engine.simulation_step();

        let mut cl_p_circle = draw_circle::Circle::new(&self.device, 0., 0., 0.05, [0., 1., 0., 1.]);
        let mut mouse_circle = draw_circle::Circle::new(&self.device, self.line_past.p2[0], self.line_past.p2[1], 0.05, [0.9, 0.6, 0., 1.]);
        let mut mouse_circle_col = circle_body::Circle::new(0, self.line_past.p2[0], self.line_past.p2[1], 0.05, 0.5);

        //let rect = Convex::new(&device, 0.5, 0.5, vec![(-0.1, 0.1), (0.1, 0.1), (0.1, -0.1), (-0.1, -0.1), (-0.15, 0.0)], [0.1, 0.0, 0.0, 1.0]);

        let mut draw_circles_iter = self.circles.iter_mut();
        self.physics_engine.physics_objects.iter().filter_map(
            |c| {
                if let CollisionType::Circle(p_c) = c.get_col_type(){
                    Some(p_c)
                }else{None}
            }
        ).for_each(
            |p_c|{
                if let Some(d_c) = draw_circles_iter.next(){
                    d_c.translate(&self.device, p_c.pos.x, p_c.pos.y);
                }
            }
        );

        self.physics_engine.physics_objects.iter().for_each(
            |p_c| {
                if let CollisionType::Convex(_c) = p_c.get_col_type(){

                    self.rect.rotate(&self.device, p_c.get_angle().0);
                    self.rect.translate(&self.device, p_c.get_pos().x, p_c.get_pos().y);
                }
            }
        );

        let rect_rp = Rect::create_render_pipeline(&self.device, &self.config);

        let mut velocity_lines : Vec<draw_line::LineMesh> = Vec::with_capacity(self.physics_engine.physics_objects.len());

        for pobj in self.physics_engine.physics_objects.iter(){
            let (x1, y1) = (pobj.get_pos().x, pobj.get_pos().y);
            let (x2, y2) = (pobj.get_pos()+pobj.get_vel()).into();
            velocity_lines.push(draw_line::LineMesh::new(&self.device, vec![[x1, y1], [x2, y2]], 0.006, [0.6, 1.0, 0.734, 1.0]));
        }


        {
        let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor{
                label: Some("render_pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                       view: &view,
                       resolve_target: None,
                       ops: wgpu::Operations {
                          load: wgpu::LoadOp::Clear(wgpu::Color {
                             r: 0.1, g: 0.2, b: 0.3, a: 1.0,
                          }),
                          store: wgpu::StoreOp::Store,
                       },


                     })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            });

        Line::draw_line(& vs, & is, & line_rp, & mut render_pass);
        LineMesh::draw_line(
            &self.line_mesh.vertex_buffer,
            &self.line_mesh.index_buffer,
            self.line_mesh.num_indices,
            &line_rp,
            &mut render_pass
        );

        for c in self.circles.iter(){
            c.draw(&self.circle_rp, &mut render_pass);
        }

        self.rect.draw(&rect_rp, &mut render_pass);

        cl_p_circle.draw(&self.circle_rp, &mut render_pass);
        mouse_circle.draw(&self.circle_rp, &mut render_pass);

        for vel_line in velocity_lines.iter(){
            vel_line.draw(&line_rp, &mut render_pass);
        }
/*
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0,
                                      //vs.slice(..)
                                      self.vertex_buffer.slice(..)
                                      );
        render_pass.set_index_buffer(
            //is.slice(..),
            self.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
*/
/*
        render_pass.set_bind_group(0, &sprite.bind_group, &[]);
        render_pass.set_vertex_buffer(0, sprite.vertex_buffer.slice(..));
        render_pass.set_index_buffer(sprite.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..sprite.num_elements, 0, 0..1);
        */
        }


        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        if let Ok(elapsed_time) = last_systemtime.elapsed(){
            if Self::delta_time > elapsed_time{
                std::thread::sleep(Self::delta_time - elapsed_time);
            }
            else{
                println!("delta < elapsed time");
                std::thread::sleep(Self::delta_time);
            }
        }
        else{
            println!("Duration error");
            std::thread::sleep(Self::delta_time);
        }

        Ok(())

    }

    fn draw_new_line<'a, 'b>(
        &'a self,
        render_pass: &'b mut wgpu::RenderPass<'a>,
        render_pipeline: &'a wgpu::RenderPipeline,
        line_mesh: &'a mut draw_line::LineMesh,
        x1: f32, y1: f32, x2: f32, y2: f32, //color: [f32; 4]
    ){
        line_mesh.update_mesh(&self.device, vec![[x1, y1], [x2, y2]]);
        line_mesh.draw(render_pipeline, render_pass);
    }


}
