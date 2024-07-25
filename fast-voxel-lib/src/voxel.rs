use cgmath::*;
use bytemuck::{Zeroable, Pod};

pub struct VoxelSpace {
    trees: Vec<VoxelTree>
}

pub struct VoxelTree {
    root_node: Node,
    rotation: Matrix4<f32>,
    position: Vector3<f32>,

    size: usize
}

pub struct Node {
    material: Option<Material>,
    childs: Option<Box<Node>>
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct Material {
    pub emmitance: [f32; 4],
    pub reflectance: [f32; 4],
    pub roughness: f32,
    pub opacity: f32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Zeroable, Pod)]
pub struct CompiledNode {
    pub position: [f32; 4],
    pub rotation: [[f32; 4]; 4],
    pub size: f32,

    pub material: Material,
    pub childs: [u32; 8],
    pub is_leaf: u32
}