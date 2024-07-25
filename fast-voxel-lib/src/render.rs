use std::mem;

#[allow(unused_imports)]
use log::*;
use wgpu::*;
use cgmath::*;
use wgpu::util::*;

use wgpu::util::DeviceExt;

use winit::{dpi::PhysicalSize, event::*};
use bytemuck::{Zeroable, Pod};

use crate::{voxel::{self, CompiledNode, Material}, App};


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
struct MetaDataUniform {
    res: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
struct TimeUniform {
    time: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
struct CameraMatrixUniform {
    matrix: [f32; 4]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
struct CameraPositionUniform {
    pos: [f32; 4]
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
    pub camera: Camera
}

pub struct Render {
    render_pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    meta_data_buffer: Buffer,
    meta_data_uniform: MetaDataUniform,
    meta_data_bind_group: BindGroup,

    time_buffer: Buffer,
    time_uniform: TimeUniform,
    time_bind_group: BindGroup,

    camera_pos_buffer: Buffer,
    camera_pos_uniform: CameraPositionUniform,
    camera_pos_bind_group: BindGroup,

    camera_rot_buffer: Buffer,
    camera_rot_uniform: CameraMatrixUniform,
    camera_rot_bind_group: BindGroup,

    camera: Camera,
    camera_controller: CameraController,

    voxel_buffer: Buffer,
    voxel_list: Vec<voxel::CompiledNode>,
    voxel_bind_group: BindGroup
}

impl Render {
    pub fn new(desc: RenderCreateDescriptor, app: &App) -> Self {
        let shader = desc.shader.shader_module;

        let meta_data_uniform = MetaDataUniform {
            res: [app.size.width as f32, app.size.height as f32],
        };

        let time_uniform = TimeUniform {
            time: 0
        };

        let camera_pos_uniform = CameraPositionUniform {
            pos: [
                desc.camera.build().1.x,
                desc.camera.build().1.y,
                desc.camera.build().1.z,
                0.0
            ]
        };

        let camera_rot_uniform = CameraMatrixUniform {
            matrix: [
                desc.camera.build().0.x,
                desc.camera.build().0.y,
                desc.camera.build().0.z,
                0.0
            ]
        };

        let voxel_list = vec![CompiledNode {
            position: [0.0, 0.0, 0.0, 0.0],
            rotation: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ],
            size: 1.0,
            material: Material {
                emmitance: [1.0, 1.0, 1.0, 0.0],
                reflectance: [0.0, 0.0, 0.0, 0.0],
                roughness: 0.0,
                opacity: 0.0
            },
            childs: [!0u32; 8],
            is_leaf: 0
        }];

        let meta_data_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Res uniform (buffer)"),
            contents: bytemuck::cast_slice(&[meta_data_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let time_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Time uniform (buffer)"),
            contents: bytemuck::cast_slice(&[time_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let camera_pos_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform (buffer)"),
            contents: bytemuck::cast_slice(&[camera_pos_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let camera_rot_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform (buffer)"),
            contents: bytemuck::cast_slice(&[camera_rot_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let voxel_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Voxel buffer"),
            contents: bytemuck::cast_slice(voxel_list.as_slice()),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
        });

        let meta_data_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Res bind group layout")
        });

        let time_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Time bind group layout")
        });

        let camera_pos_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Camera bind group layout")
        });

        let camera_rot_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Camera bind group layout")
        });

        let voxel_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Voxel bind group layout")
        });

        let render_pipeline_layout = app.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[
                &meta_data_bind_group_layout,
                &time_bind_group_layout,
                &camera_pos_bind_group_layout,
                &camera_rot_bind_group_layout,
                &voxel_bind_group_layout
            ],
            push_constant_ranges: &[]
        });

        let meta_data_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &meta_data_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: meta_data_buffer.as_entire_binding()
                }
            ],
            label: Some("Res bind group")
        });

        let time_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &time_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: time_buffer.as_entire_binding()
                }
            ],
            label: Some("Time bind group")
        });

        let camera_pos_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_pos_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_pos_buffer.as_entire_binding()
                }
            ],
            label: Some("Camera bind group")
        });

        let camera_rot_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_rot_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_rot_buffer.as_entire_binding()
                }
            ],
            label: Some("Camera bind group")
        });

        let voxel_bind_group = app.device.create_bind_group(&BindGroupDescriptor {
            layout: &voxel_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: voxel_buffer.as_entire_binding()
                }
            ],
            label: Some("Voxel bind group")
        });

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

        let camera_controller = CameraController::new(CameraControllerCreateDescriptor {
            speed: 15.0
        }, &app);

        Self {
            render_pipeline,
            vertex_buffer,
            meta_data_bind_group,
            meta_data_buffer,
            meta_data_uniform,
            time_bind_group,
            time_buffer,
            time_uniform,
            camera_pos_bind_group,
            camera_pos_buffer,
            camera_pos_uniform,
            camera_rot_bind_group,
            camera_rot_buffer,
            camera_rot_uniform,
            camera: desc.camera,
            camera_controller,
            voxel_bind_group,
            voxel_buffer,
            voxel_list
        }
    }

    fn render(&self, app: &App) -> Result<(), SurfaceError> {
        let output = app.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = app.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Command encoder")
        });

        #[allow(unused_variables)]
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

            render_pass.set_bind_group(0, &self.meta_data_bind_group, &[]);
            render_pass.set_bind_group(1, &self.time_bind_group, &[]);
            render_pass.set_bind_group(2, &self.camera_pos_bind_group, &[]);
            render_pass.set_bind_group(3, &self.camera_rot_bind_group, &[]);
            render_pass.set_bind_group(4, &self.voxel_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }

        app.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn resize(&mut self, physical_size: PhysicalSize<u32>, app: &mut App) {
        if physical_size.width <= 0 || physical_size.height <= 0 { return; }

        app.size = physical_size;
        app.surface_config.width = physical_size.width;
        app.surface_config.height = physical_size.height;
        app.surface.configure(&app.device, &app.surface_config);

        self.meta_data_uniform.res = [app.size.width as f32, app.size.height as f32];
        app.queue.write_buffer(
            &self.meta_data_buffer,
            0,
            bytemuck::cast_slice(&[self.meta_data_uniform])
        );
    }

    fn update(&mut self, app: &App) {
        self.time_uniform.time += 1;

        if self.time_uniform.time == 1000 {
            self.time_uniform.time = 0;
        }

        app.queue.write_buffer(
            &self.time_buffer,
            0,
            bytemuck::cast_slice(&[self.time_uniform])
        );
    }

    pub fn handle_events(&mut self, event: &winit::event::WindowEvent, app: &mut App) {
        if self.camera_controller.handle_events(event) {
            self.camera_controller.handle_camera(&mut self.camera, &app);

            self.camera_pos_uniform.pos = [
                self.camera.build().1.x,
                self.camera.build().1.y,
                self.camera.build().1.z,
                0.0
            ];

            app.queue.write_buffer(
                &self.camera_pos_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_pos_uniform])
            );

            self.camera_rot_uniform.matrix = [
                self.camera.build().0.x,
                self.camera.build().0.y,
                self.camera.build().0.z,
                0.0
            ];

            app.queue.write_buffer(
                &self.camera_rot_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_rot_uniform])
            );
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

pub struct CameraCreateDescriptor {
    pub pos: Point3<f32>,
    pub rot: Vector3<f32>
}

pub struct Camera {
    pos: Point3<f32>,
    rot: Vector3<f32>,
}

impl Camera {
    #[allow(unused_variables)]
    pub fn new(desc: &CameraCreateDescriptor, app: &App) -> Self {
        Self {
            pos: desc.pos,
            rot: desc.rot
        }
    }

    pub fn build(&self) -> (Vector3<f32>, Point3<f32>) {
        (
            // Matrix4::from_angle_x(Rad(self.rot.x)) *
            // Matrix4::from_angle_y(Rad(self.rot.x)) *
            // Matrix4::from_angle_z(Rad(self.rot.x)),
            self.rot,
            self.pos
        )
    }
}

pub struct CameraController {
    speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    is_left_rot_pressed: bool,
    is_right_rot_pressed: bool
}

pub struct CameraControllerCreateDescriptor {
    pub speed: f32
}

impl CameraController {
    #[allow(unused_variables)]
    pub fn new(desc: CameraControllerCreateDescriptor, app: &App) -> Self {
        Self {
            speed: desc.speed,

            is_backward_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,

            is_left_rot_pressed: false,
            is_right_rot_pressed: false
        }
    }

    pub fn handle_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: winit::keyboard::PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    winit::keyboard::KeyCode::KeyW | winit::keyboard::KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::KeyS | winit::keyboard::KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::KeyD => {
                        self.is_right_pressed = is_pressed;
                        true
                    }

                    winit::keyboard::KeyCode::ArrowLeft => {
                        self.is_left_rot_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::ArrowRight => {
                        self.is_right_rot_pressed = is_pressed;
                        true
                    }

                    _ => false,
                }
            }
            _ => false,
        }
    }

    #[allow(unused_variables)]
    pub fn handle_camera(&self, camera: &mut Camera, app: &App) {
        let sin_ax = camera.rot.x.sin();
        let cos_ax = camera.rot.x.cos();

        let sin_az = camera.rot.z.sin();
        let cos_az = camera.rot.z.cos();

        if self.is_left_rot_pressed {
            camera.rot.x += self.speed * app.delta_time as f32;
        }
        if self.is_right_rot_pressed {
            camera.rot.x -= self.speed * app.delta_time as f32;
        }

        if self.is_forward_pressed {
            camera.pos.x += cos_ax * app.delta_time as f32 * self.speed;
            camera.pos.y += sin_ax * app.delta_time as f32 * self.speed;
        }
        if self.is_backward_pressed {
            camera.pos.x -= cos_ax * app.delta_time as f32 * self.speed;
            camera.pos.y -= sin_ax * app.delta_time as f32 * self.speed;
        }
        if self.is_left_pressed {
            camera.pos.x += sin_ax * app.delta_time as f32 * self.speed;
            camera.pos.y -= cos_ax * app.delta_time as f32 * self.speed;
        }
        if self.is_right_pressed {
            camera.pos.x -= sin_ax * app.delta_time as f32 * self.speed;
            camera.pos.y += cos_ax * app.delta_time as f32 * self.speed;
        }
    }
}