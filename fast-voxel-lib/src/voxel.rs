use std::fs;

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
    uniform: Vec<CompiledUniform>,
    uniform_buffer: Buffer,
    uniform_bind_group_layout: BindGroupLayout,
    uniform_bind_group: Option<BindGroup>,
}

impl VoxelTree {
    pub fn new(app: &App, binding: u32, max_nodes: usize) -> Self {
        let uniform = vec![];

        let uniform_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform (buffer)"),
            contents: vec![0; max_nodes * 44 * 4].as_slice(),
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

    pub fn load(&mut self, file: String) {
        let file = fs::read_to_string(file)
            .expect("Error to load scene file");

        let json: serde_json::Value = serde_json::from_str(file.as_str())
            .expect("Error to load scene file");

        for object in json.as_array().unwrap() {
            self.uniform.push(CompiledUniform {
                position: [
                    object["pos"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                    object["pos"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                    object["pos"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                ],
                rotation: [
                    [
                        object["rot"].as_array().unwrap()[0][0].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[0][1].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[0][2].as_f64().unwrap() as f32,
                    ],
                    [
                        object["rot"].as_array().unwrap()[1][0].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[1][1].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[1][2].as_f64().unwrap() as f32,
                    ],
                    [
                        object["rot"].as_array().unwrap()[2][0].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[2][1].as_f64().unwrap() as f32,
                        object["rot"].as_array().unwrap()[2][2].as_f64().unwrap() as f32,
                    ]
                ],
                size: object["size"].as_f64().unwrap() as f32,
                material: MaterialUniform {
                    emmitance: [
                        object["material"]["emmitance"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                        object["material"]["emmitance"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                        object["material"]["emmitance"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                    ],
                    reflectance: [
                        object["material"]["reflectance"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                        object["material"]["reflectance"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                        object["material"]["reflectance"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                    ],

                    roughness: object["material"]["roughness"].as_f64().unwrap() as f32,
                    opacity: object["material"]["opacity"].as_f64().unwrap() as f32,
                },
                childs: [
                    object["childs"].as_array().unwrap()[0].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[1].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[2].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[3].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[4].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[5].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[6].as_f64().unwrap() as f32,
                    object["childs"].as_array().unwrap()[7].as_f64().unwrap() as f32,
                ],
                is_leaf: object["is_leaf"].as_f64().unwrap() as f32,
                _offset: [0.0; 14]
            })
        }
    }

    pub fn update_buffers(&self, app: &App) {
        app.queue.write_buffer(
            &self.uniform_buffer,
            0,
            {
                let mut out = Vec::<u8>::new();

                for exp in (&self.uniform).into_iter() {
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct MaterialUniform {
    pub emmitance: [f32; 3],
    pub reflectance: [f32; 3],
    pub roughness: f32,
    pub opacity: f32
}

impl Default for MaterialUniform {
    fn default() -> Self {
        Self {
            emmitance: [1.0, 1.0, 1.0],
            reflectance: [1.0, 1.0, 1.0],
            roughness: 0.0,
            opacity: 0.0,
        }
    }
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