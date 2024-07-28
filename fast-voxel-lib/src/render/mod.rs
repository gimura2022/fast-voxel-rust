pub mod camera;

use std::mem;

use log::*;
use wgpu::*;
use cgmath::*;
use wgpu::util::*;

use wgpu::util::DeviceExt;

use winit::{dpi::PhysicalSize, event::*};
use bytemuck::{Pod, Zeroable};

use crate::{voxel::VoxelTree, App};
use camera::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 3]
}

impl Vertex {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, 0.0] },
    Vertex { position: [1.0, -1.0, 0.0] },

    Vertex { position: [1.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 0.0] },
    Vertex { position: [1.0, -1.0, 0.0] },
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
struct MetaDataUniformRaw {
    res: [f32; 2],
    time: u32,

    _offset: [u32; 4]
}

struct MetaDataUniform {
    uniform: MetaDataUniformRaw,
    uniform_buffer: Buffer,
    uniform_bind_group_layout: BindGroupLayout,
    uniform_bind_group: Option<BindGroup>
}

impl MetaDataUniform {
    pub fn new(raw: MetaDataUniformRaw, binding: u32, app: &App) -> Self {
        let uniform = raw;

        let uniform_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Meta data uniform (buffer)"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let uniform_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Meta data bind group layout")
        });

        Self {
            uniform,
            uniform_bind_group_layout,
            uniform_buffer,
            uniform_bind_group: None,
        }
    }

    pub fn uniform_bind_group_layout(&self) -> &BindGroupLayout {
        &self.uniform_bind_group_layout
    }

    pub fn uniform_bind_group(&self) -> &BindGroup {
        &self.uniform_bind_group.as_ref().unwrap()
    }

    pub fn init(&mut self, binding: u32, app: &App) {
        let uniform_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &self.uniform_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding,
                    resource: self.uniform_buffer.as_entire_binding()
                }
            ],
            label: Some("Meta data bind group")
        });

        self.uniform_bind_group = Some(uniform_bind_group);
    }

    pub fn uniform(&self) -> &MetaDataUniformRaw {
        &self.uniform
    }

    pub fn update(&mut self, uniform: MetaDataUniformRaw, app: &App) {
        self.uniform = uniform;

        app.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniform])
        )
    }
}

pub struct ShaderCreateDescriptor {
    pub shdaer_source: String
}

pub struct Shader {
    shader_module: ShaderModule
}

impl Shader {
    pub fn new(desc: &ShaderCreateDescriptor, app: &App) -> Self {
        let shader_module = app.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(desc.shdaer_source.as_str().into())
        });

        Self {
            shader_module
        }
    }
}

pub struct RenderCreateDescriptor {
    pub shader: Shader,
    pub camera: (Point3<f32>, Vector3<f32>)
}

pub struct Render {
    render_pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    meta_data: MetaDataUniform,
    voxel_tree: VoxelTree,

    camera: Camera,
    camera_controller: CameraController,
}

impl Render {
    pub fn new(desc: RenderCreateDescriptor, app: &App) -> Self {
        let shader = desc.shader.shader_module;

        let mut camera = Camera::new(desc.camera.0, desc.camera.1, 0, app);

        let mut meta_data = MetaDataUniform::new(MetaDataUniformRaw {
            res: [app.size.width as f32, app.size.height as f32],
            time: 0,
            _offset: [0; 4]
        }, 0, app);

        let mut voxel_tree = VoxelTree::new(app, 0, 2629);
        voxel_tree.load("scene_vox.json".to_string());
       
        let render_pipeline_layout = app.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[
                camera.uniform_bind_group_layout(),
                meta_data.uniform_bind_group_layout(),
                voxel_tree.uniform_bind_group_layout(),
            ],
            push_constant_ranges: &[]
        });

        camera.init(0, app);
        meta_data.init(0, app);
        voxel_tree.init(0, app);

        let render_pipeline = app.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc()
                ]
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: app.surface_config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL
                })]
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        });

        let vertex_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX
        });

        let camera_controller = CameraController::new(15.0);

        Self {
            render_pipeline,
            vertex_buffer,
            meta_data,
            camera,
            camera_controller,
            voxel_tree
        }
    }

    fn render(&self, app: &App) -> Result<(), SurfaceError> {
        let output = app.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = app.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Command encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0
                        }),
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.camera.uniform_bind_group(), &[]);
            render_pass.set_bind_group(1, &self.meta_data.uniform_bind_group(), &[]);
            render_pass.set_bind_group(2, &self.voxel_tree.uniform_bind_group(), &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }

        app.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn resize(&mut self, physical_size: PhysicalSize<u32>, app: &mut App) {
        app.size = physical_size;
        app.surface_config.width = physical_size.width;
        app.surface_config.height = physical_size.height;
        app.surface.configure(&app.device, &app.surface_config);

        self.meta_data.update(MetaDataUniformRaw {
            res: [app.size.width as f32, app.size.height as f32],
            time: self.meta_data.uniform().time,
            _offset: [0; 4]
        }, app);
    }

    fn update(&mut self, app: &App) {
        self.meta_data.uniform.time += 1;

        if self.meta_data.uniform.time == 1000 {
            self.meta_data.uniform.time = 0;
        }

        self.meta_data.update(self.meta_data.uniform, app);
    }

    pub fn handle_events(&mut self, event: &winit::event::WindowEvent, app: &mut App) {
        if self.camera_controller.handle_events(event) {
            self.camera_controller.handle_camera(&mut self.camera, &app);
            self.camera.update_uniforms(&app);
        }

        match event {
            WindowEvent::RedrawRequested => {
                self.update(&app);

                match self.render(&app) {
                    Ok(_) => {},

                    Err(SurfaceError::Lost) => self.resize(app.size, app),
                    Err(e) => panic!("Surface error: {}", e)
                }
            }

            WindowEvent::Resized(physical_size) => self.resize(*physical_size, app),

            _ => {}
        }
    }
}