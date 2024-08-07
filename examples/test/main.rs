use fast_voxel_rs::*;
use fast_voxel_rs::render::*;

use winit::{event_loop::EventLoop, window::WindowBuilder};
        
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
        
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Trace).expect("Couldn't initialize logger");
        } else {
            let env = env_logger::Env::new().filter_or("RUST_LOG", "fast_voxel_rust=trace,fast_voxel_lib=trace,wgpu=warn");
            env_logger::init_from_env(env);
        }
    }   
        
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
        
    let app = App::new(AppDescriptor {  }, &window).await;
    let render = app.create_render(RenderCreateDescriptor {
        shader: app.create_shader(&ShaderCreateDescriptor {
            shdaer_source: include_str!("../../target/compiled.wgsl").to_string()
        }),
        camera: (
            (-10.0, 0.0, 0.0).into(),
            (0.0, 0.0, 0.0).into()
        )
    }); 
        
    app.run(render, event_loop);
} 

fn main() {
    pollster::block_on(run());
}