//! ifndef _uniforms_wgsl
//! define _uniforms_wgsl ""

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

struct MetaDataUniform {
    res: vec2<f32>,
    time: u32
}

struct CameraUniform {
    matrix: mat3x3<f32>,
    pos: vec3<f32>,
}

//! include "render_def"

@group(0) @binding(0) var<uniform> u_camera: CameraUniform;
@group(1) @binding(0) var<uniform> u_meta_data: MetaDataUniform;
@group(2) @binding(0) var<storage, read> b_voxels: array<Cube>;

fn get_voxel(i: u32) -> Cube {
    return b_voxels[i];
}

//! endif