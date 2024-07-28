use cgmath::*;
use util::BufferInitDescriptor;
use wgpu::*;
use wgpu::util::DeviceExt;
use log::*;
use bytemuck::{Zeroable, Pod, cast_slice};

use crate::App;

pub struct VoxelSpace {
    trees: Vec<VoxelTree>
}

pub struct VoxelTree {
    // root_node: Node,
    // rotation: Matrix4<f32>,
    // position: Vector3<f32>,

    // size: usize,

    uniform: Vec<CompiledUniform>,
    uniform_buffer: Buffer,
    uniform_bind_group_layout: BindGroupLayout,
    uniform_bind_group: Option<BindGroup>,
}

impl VoxelTree {
    pub fn new(app: &App, binding: u32) -> Self {
        let uniform = vec![
            CompiledUniform {
                position: (0.0, 0.0, 0.0).into(),
                rotation: Matrix3::identity().into(),
                size: 10.0,
                material: MaterialUniform {
                    emmitance: (1.0, 1.0, 1.0).into(),
                    reflectance: (0.0, 0.0, 0.0).into(),
                    roughness: 0.0,
                    opacity: 0.0,
                },
                childs: [!0 as f32; 8],
                is_leaf: 1.0,
                _offset: [0.0; 14]
            },
            CompiledUniform {
                position: (0.0, 0.0, -20.0).into(),
                rotation: Matrix3::identity().into(),
                size: 1.0,
                material: MaterialUniform {
                    emmitance: (0.0, 0.0, 0.0).into(),
                    reflectance: (1.0, 1.0, 1.0).into(),
                    roughness: 0.5,
                    opacity: 0.0,
                },
                childs: [!0 as f32; 8],
                is_leaf: 1.0,
                _offset: [0.0; 14]
            },
        ];

        let uniform_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform (buffer)"),
            // contents: bytemuck::cast_slice(uniform.as_slice()),
            contents: {
                let range = (0..88).into_iter();
                let mut out = Vec::<u8>::new();

                for i in range {
                    let bytes = bytemuck::cast_slice::<f32, u8>(&[i as f32]).to_owned();

                    out.push(bytes[0]);
                    out.push(bytes[1]);
                    out.push(bytes[2]);
                    out.push(bytes[3]);
                }

                out
            }.as_slice(),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
        });

        let uniform_bind_group_layout = app.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ],
            label: Some("Camera bind group layout")
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
            label: Some("Camera bind group")
        });

        self.uniform_bind_group = Some(uniform_bind_group);

        self.update_buffers(app);
    }

    pub fn update_buffers(&self, app: &App) {
        app.queue.write_buffer(
            &self.uniform_buffer,
            0,
            {
                let mut out = Vec::<u8>::new();

                for (i, exp) in (&self.uniform).into_iter().enumerate() {
                    out.append(&mut cast_slice(&[exp.position[0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.position[1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.position[2]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.rotation[0][0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[1][0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[2][0]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.rotation[0][1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[1][1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[2][1]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.rotation[0][2]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[1][2]]).to_owned());
                    out.append(&mut cast_slice(&[exp.rotation[2][2]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.size]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);
                    out.append(&mut vec![0, 0, 0, 0]);
                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.material.emmitance[0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.material.emmitance[1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.material.emmitance[2]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.material.reflectance[0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.material.reflectance[1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.material.reflectance[2]]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.material.roughness]).to_owned());
                    out.append(&mut cast_slice(&[exp.material.opacity]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);
                    out.append(&mut vec![0, 0, 0, 0]);
                    out.append(&mut vec![0, 0, 0, 0]);

                    out.append(&mut cast_slice(&[exp.childs[0]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[1]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[2]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[3]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[4]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[5]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[6]]).to_owned());
                    out.append(&mut cast_slice(&[exp.childs[7]]).to_owned());

                    out.append(&mut cast_slice(&[exp.is_leaf]).to_owned());

                    out.append(&mut vec![0, 0, 0, 0]);
                    out.append(&mut vec![0, 0, 0, 0]);
                }

                out.pop(); out.pop(); out.pop(); out.pop();
                out.pop(); out.pop(); out.pop(); out.pop();

                out
            }.as_slice()
        )
    }
}

pub struct Node {
    material: Option<MaterialUniform>,
    childs: Option<Box<Node>>
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct MaterialUniform {
    pub emmitance: [f32; 3],
    pub reflectance: [f32; 3],
    pub roughness: f32,
    pub opacity: f32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct CompiledUniform {
    pub position: [f32; 3],
    pub rotation: [[f32; 3]; 3],
    pub size: f32,

    pub material: MaterialUniform,
    pub childs: [f32; 8],
    pub is_leaf: f32,
    pub _offset: [f32; 14]
}