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
    time: u32
}

struct CameraUniform {
    matrix: mat3x3<f32>,
    pos: vec3<f32>,
}

struct Cube {
    position: vec3<f32>,
    rotation: mat3x3<f32>,
    size: f32,

    material: Material,
    childs: array<f32, 8>,
    is_leaf: f32,
}

struct Material {
    emmitance: vec3<f32>,
    reflectance: vec3<f32>,
    roughness: f32,
    opacity: f32
}

@group(0) @binding(0) var<uniform> u_camera: CameraUniform;
@group(1) @binding(0) var<uniform> u_meta_data: MetaDataUniform;

@group(2) @binding(0) var<storage, read> b_voxels: array<Cube>;

//! endif