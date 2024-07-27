use bytemuck::Zeroable;
use bytemuck::Pod;
use bytemuck::cast_slice;
use util::BufferInitDescriptor;
use util::DeviceExt;
use winit::event::ElementState;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use wgpu::*;
use cgmath::*;
use log::*;

use crate::App;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct CameraUniform {
    pub(crate) rot: [[f32; 3]; 3],
    pub(crate) pos: [f32; 3],
}

#[derive(Debug)]
pub struct Camera {
    pos: Point3<f32>,
    rot: Vector3<f32>,

    uniform: CameraUniform,
    uniform_buffer: Buffer,
    uniform_bind_group_layout: BindGroupLayout,
    uniform_bind_group: Option<BindGroup>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, rot: Vector3<f32>, binding: u32, app: &App) -> Self {
        let uniform = CameraUniform {
            pos: [0.0; 3],
            rot: [[0.0; 3]; 3],
        };

        let uniform_buffer = app.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform (buffer)"),
            // contents: bytemuck::cast_slice(&[uniform]),
            contents: {
                let range = (0..16).into_iter();
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
            label: Some("Camera bind group layout")
        });

        Self {
            pos,
            rot,

            uniform,
            uniform_buffer,
            uniform_bind_group_layout,
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
        self.uniform = self.build();
        self.update_uniforms(app);
    }

    pub fn update_uniforms(&mut self, app: &App) {
        self.uniform = self.build();

        app.queue.write_buffer(
            &self.uniform_buffer,
            0,
            {
                let mut out = Vec::<u8>::new();

                trace!("rotate: {:#?}", self.uniform.rot);

                out.append(&mut cast_slice(&[self.uniform.rot[0][0]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[1][0]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[2][0]]).to_owned());

                out.append(&mut vec![0, 0, 0, 0]);

                out.append(&mut cast_slice(&[self.uniform.rot[0][1]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[1][1]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[2][1]]).to_owned());

                out.append(&mut vec![0, 0, 0, 0]);

                out.append(&mut cast_slice(&[self.uniform.rot[0][2]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[1][2]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.rot[2][2]]).to_owned());

                out.append(&mut vec![0, 0, 0, 0]);

                out.append(&mut cast_slice(&[self.uniform.pos[0]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.pos[1]]).to_owned());
                out.append(&mut cast_slice(&[self.uniform.pos[2]]).to_owned());

                out.append(&mut vec![0, 0, 0, 0]);

                trace!("{:?}", self.uniform.pos);

                out
            }.as_slice()
        );
    }

    fn build(&self) -> CameraUniform {
        trace!("{:?}", self.rot);

        CameraUniform {
            rot: {
                Matrix3::identity() *
                Matrix3::from_angle_x(Rad(-self.rot.x)) *
                Matrix3::from_angle_y(Rad(-self.rot.y)) *
                Matrix3::from_angle_z(Rad(-self.rot.z))
            }.into(),
            pos: self.pos.into(),
        }
    }
}

pub struct CameraController {
    speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    is_left_rot_pressed: bool,
    is_right_rot_pressed: bool,
    is_up_rot_pressed: bool,
    is_down_rot_pressed: bool
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,

            is_backward_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,

            is_left_rot_pressed: false,
            is_right_rot_pressed: false,
            is_down_rot_pressed: false,
            is_up_rot_pressed: false
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
                    winit::keyboard::KeyCode::KeyW => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::KeyS => {
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
                    winit::keyboard::KeyCode::ArrowUp => {
                        self.is_up_rot_pressed = is_pressed;
                        true
                    }
                    winit::keyboard::KeyCode::ArrowDown => {
                        self.is_down_rot_pressed = is_pressed;
                        true
                    }

                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn handle_camera(&self, camera: &mut Camera, app: &App) {
        let sin_az = camera.rot.z.sin();
        let cos_az = camera.rot.z.cos();

        if self.is_left_rot_pressed {
            camera.rot.z += self.speed / 2.0 * app.delta_time as f32;
        }
        if self.is_right_rot_pressed {
            camera.rot.z -= self.speed / 2.0 * app.delta_time as f32;
        }
        if self.is_up_rot_pressed {
            camera.rot.y += self.speed / 2.0 * app.delta_time as f32;
        }
        if self.is_down_rot_pressed {
            camera.rot.y -= self.speed / 2.0 * app.delta_time as f32;
        }

        if self.is_forward_pressed {
            camera.pos.x += self.speed * cos_az * app.delta_time as f32;
            camera.pos.y += self.speed * sin_az * app.delta_time as f32;
        }
        if self.is_backward_pressed {
            camera.pos.x += -self.speed * cos_az * app.delta_time as f32;
            camera.pos.y += -self.speed * sin_az * app.delta_time as f32;
        }
        if self.is_left_pressed {
            camera.pos.x += self.speed * sin_az * app.delta_time as f32;
            camera.pos.y += -self.speed * cos_az * app.delta_time as f32;
        }
        if self.is_right_pressed {
            camera.pos.x += -self.speed * sin_az * app.delta_time as f32;
            camera.pos.y += self.speed * cos_az * app.delta_time as f32;
        }
    }
}