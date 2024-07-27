//! ifndef _std_wgsl
//! define _std_wgsl ""

//! ifndef _eng_header_wgsl
//! define _eng_header_wgsl ""

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

//! ifndef _render_def_wgsl
//! define _render_def_wgsl ""

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

struct IntersectInfo {
    is_intersected: bool,
    fraction: f32,
    normal: vec3<f32>,
    material: Material
}

fn box_int(_ro: vec3<f32>, _rd: vec3<f32>, cube: Cube) -> IntersectInfo {
    var out: IntersectInfo;

    let out_rot = mat3x3<f32>(
        cube.rotation.x.xyz,
        cube.rotation.y.xyz,
        cube.rotation.z.xyz,
    );

    let out_pos = cube.position.xyz;

    let rd = out_rot * _rd;
    let ro = out_rot * (_ro - out_pos);

    let m = vec3<f32>(1.0) / rd;

    var s = vec3<f32>(0.0);
    if rd.x < 0.0 { s.x = 1.0; } else { s.x = -1.0; }
    if rd.y < 0.0 { s.y = 1.0; } else { s.y = -1.0; }
    if rd.z < 0.0 { s.z = 1.0; } else { s.z = -1.0; }

    let t1 = m * (-ro + s * cube.size);
    let t2 = m * (-ro - s * cube.size);

    let tN = max(max(t1.x, t1.y), t1.z);
    let tF = min(min(t2.x, t2.y), t2.z);

    if tN > tF || tF < 0.0 {
        out.is_intersected = false;
        return out;
    }

    let txi = transpose(out_rot);

    if t1.x > t1.y && t1.x > t1.z { out.normal = txi[0] * s.x; }
    else if t1.y > t1.z           { out.normal = txi[1] * s.y; }
    else                          { out.normal = txi[2] * s.z; }

    out.fraction = tN;
    out.is_intersected = true;

    return out;
}

//! endif
//! include "render_def"

@group(0) @binding(0) var<uniform> u_camera: CameraUniform;
@group(1) @binding(0) var<uniform> u_meta_data: MetaDataUniform;
@group(2) @binding(0) var<storage, read> b_voxels: array<Cube>;

//! endif
//! include "uniforms"
//! ifndef _vertex_wgsl
//! define _vertex_wgsl ""




































































































//! include "uniforms"

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = vec2<f32>(model.position.x, model.position.y);

    return out;
}

//! endif
//! include "vertex"

//! endif
//! include "header"
//! ifndef _rand_wgsl
//! define _rand_wgsl ""




























































































































































































































//! include "header"

var<private> seed: f32 = 8498201.443902;

fn _rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn rand(_co: vec2<f32>) -> f32 {
    let r = _rand(_co * f32(seed));

    let seed_f = r * 12.774;
    seed = seed_f;

    return r;
}

fn rand2(_co: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(rand(_co), rand(_co));
}

fn rand3(_co: vec2<f32>) -> vec3<f32> {
    return vec3<f32>(rand(_co), rand(_co), rand(_co));
}

fn rand4(_co: vec2<f32>) -> vec4<f32> {
    return vec4<f32>(rand(_co), rand(_co), rand(_co), rand(_co));
}

//! endif
//! include "rand"
//! ifndef _render_wgsl
//! define _render_wgsl ""








































































//! include "render_def"
//! ifndef _render_ray_casting_wgsl
//! define _render_ray_casting_wgsl ""








































































//! include "render_def"



































































































//! include "uniforms"

//! define FAR_DISTANCE "1000000.0"

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>) -> IntersectInfo {
    var out: IntersectInfo;
var min_dist = 1000000.0;

    for (var i = 0; i < 2; i++) {
        let int = box_int(ro, rd, b_voxels[i]);

        if int.is_intersected && int.fraction < min_dist {
            min_dist = int.fraction;
            out.normal = int.normal;
            out.material = b_voxels[i].material;
        }
    }

    out.fraction = min_dist;
out.is_intersected = 1000000.0 != min_dist;

    return out;
}

fn get_color_with_ray_casting(ro: vec3<f32>, rd: vec3<f32>) -> vec3<f32> {
    let hit = cast_ray(ro, rd);

    if hit.is_intersected {
        return hit.material.reflectance;
    } else {
        return vec3<f32>(0.0);
    }
}
 
//! endif
//! include "render_ray_casting"
//! ifndef _render_ray_trasing_wgsl
//! define _render_ray_trasing_wgsl ""

//! define MAX_DEPTH "4"








































































//! include "render_def"
















































































































































































































//! include "render_ray_casting"
//! ifndef _math_wgsl
//! define _math_wgsl ""

//! define PI "3.141592653"

//! endif
//! include "math"



























































































































































































































































//! include "rand"

fn rand_point(rand: vec2<f32>) -> vec3<f32> {
    let cos_theta = sqrt(1.0 - rand.x);
    let sin_theta = sqrt(rand.x);
let phi = 2.0 * 3.141592653 * rand.y;

    return vec3<f32>(
        cos(phi) * sin_theta,
        sin(phi) * sin_theta,
        cos_theta
    );
}

fn normal_point(rand: vec2<f32>, n: vec3<f32>) -> vec3<f32> {
    let v = rand_point(rand);

    if dot(v, n) < 0.0 {
        return -v;
    }

    return v;
}

fn trace_ray(_ro: vec3<f32>, _rd: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
    var L = vec3<f32>(0.0);
    var F = vec3<f32>(1.0);

    var ro = _ro;
    var rd = _rd;

for (var i = 0; i < 4; i++) {
        let hit = cast_ray(ro, rd);

        if hit.is_intersected {
            var new_ro = ro + hit.fraction * rd;
            
            let dist_dir = normal_point(rand2(uv), hit.normal);
            let rand_vec = normalize(2.0 * rand3(uv) - 1.0);
            
            let tangent = cross(rand_vec, hit.normal);
            let bitangent = cross(hit.normal, tangent);
            let transform = mat3x3<f32>(tangent, bitangent, hit.normal);

            var new_rd = transform * dist_dir;

            let ideal_ref = reflect(rd, hit.normal);
            new_rd = normalize(mix(new_rd, ideal_ref, hit.material.roughness));

            new_ro += hit.normal * 0.8;

            rd = new_rd;
            ro = new_ro;

            L += F * hit.material.emmitance.xyz;
            F *= hit.material.reflectance.xyz;
        } else {
            F = vec3<f32>(0.0);
        }
    }

    return L;
}

//! endif
//! include "render_ray_trasing"

//! endif
//! include "render"





//! include "math"

//! endif
//! include "std"

//! define SAMPLE_COUNT "10"

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * u_meta_data.res / u_meta_data.res.y;
    
    var ray_orig = u_camera.pos.xyz;
    let ray_dir = (normalize(vec3<f32>(1.0, uv)) * u_camera.matrix).xyz;

    var color = vec3<f32>(0.0);

for (var sample = 0; sample < 10; sample++) {
        seed = (f32(sample) + uv * 18.23189).x;
        let tmp_color = trace_ray(ray_orig, ray_dir, uv + f32(sample) * 23.00231);
        color += tmp_color;
    }

color /= f32(10);

    return vec4<f32>(color, 1.0);
}