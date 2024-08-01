pub mod render;
pub mod voxel;

use std::time::Instant;

use log::*;
use wgpu::*;

use winit::{
    dpi::*, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::*
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub struct AppDescriptor {

} 

#[allow(dead_code)]
pub struct App<'a> {
    instance: Instance,
    
    device: Device,
    queue: Queue,

    surface: Surface<'a>,
    surface_config: SurfaceConfiguration,

    size: PhysicalSize<u32>,

    window: &'a Window,
    delta_time: f64
}

impl<'a> App<'a> {
    #[allow(unused_variables)]
    pub async fn new(desc: AppDescriptor, window: &'a Window) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-root")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            #[cfg(not(target_arch="wasm32"))]
            backends: Backends::PRIMARY,
            #[cfg(target_arch="wasm32")]
            backends: Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();

        debug!("Selected device: {}", adapter.get_info().name);

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window,
            instance,
            device,
            queue,
            surface,
            surface_config,
            size,
            delta_time: 0.0
        }
    }

    pub fn create_render(&self, desc: render::RenderCreateDescriptor) -> render::Render {
        render::Render::new(desc, &self)
    }

    pub fn create_shader(&self, desc: &render::ShaderCreateDescriptor) -> render::Shader {
        render::Shader::new(desc, &self)
    }

    #[allow(unused_assignments)]
    pub fn run(mut self, mut render: render::Render, event_loop: EventLoop<()>) {
        let mut current_time = Instant::now();

        event_loop.run(move |event, control_flow| {
            let new_time = Instant::now();

            // trace!("delta: {}, new_time: {:?}, current_time: {:?}",
            //     self.delta_time,
            //     new_time,
            //     current_time
            // );

            // trace!("fps: {}, delta: {}", 1.0 / self.delta_time, self.delta_time);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    render.handle_events(event, &mut self);

                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),

                        _ => {}
                    }
                },

                Event::AboutToWait => self.window.request_redraw(),

                _ => {}
            }

            self.delta_time = current_time.elapsed().as_secs_f64() - new_time.elapsed().as_secs_f64();
            current_time = new_time;
        }).unwrap();
    }
}