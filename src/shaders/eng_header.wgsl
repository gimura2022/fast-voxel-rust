//! ifndef _eng_header_wgsl
//! define _eng_header_wgsl ""

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

struct MetaDataUniform {
    res: vec2<f32>,
}

struct TimeUniform {
    time: u32
}

struct CameraMatrixUniform {
    matrix: mat4x4<f32>
}

struct CameraPositionUniform {
    pos: vec4<f32>
}

@group(0) @binding(0) var<uniform> u_meta_data: MetaDataUniform;
@group(1) @binding(0) var<uniform> u_time: TimeUniform;
@group(2) @binding(0) var<uniform> u_cam_pos: CameraPositionUniform;
@group(3) @binding(0) var<uniform> u_cam_rot: CameraMatrixUniform;

//! endif