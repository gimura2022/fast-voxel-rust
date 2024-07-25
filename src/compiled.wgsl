//! ifndef _ray_trasing_wgsl
//! define _ray_trasing_wgsl ""

struct Cube {
    size: f32,
    material: Material,
    position: vec3<f32>,
    rotation: mat3x3<f32>
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

fn sph_int(ro: vec3<f32>, rd: vec3<f32>, cube: Cube) -> IntersectInfo {
    var out: IntersectInfo;

    let L = ro - cube.position;
    let a = dot(rd, rd);
    let b = 2.0 * dot(L, rd);
    let c = dot(L, L) - cube.size * cube.size;
    let D = b * b - 4 * a * c;

    if D < 0.0 {
        out.is_intersected = false;
        return out;
    }

    let r1 = (-b - sqrt(D)) / (2.0 * a);
    let r2 = (-b + sqrt(D)) / (2.0 * a);

    if r1 > 0.0 {
        out.fraction = r1;
    } else if r2 > 0.0 {
        out.fraction = r2;
    } else {
        out.is_intersected = false;
        return out;
    }

    out.normal = normalize(rd * out.fraction + L);
    out.is_intersected = true;

    return out;
}

fn box_int(_ro: vec3<f32>, _rd: vec3<f32>, cube: Cube) -> IntersectInfo {
    var out: IntersectInfo;

    let rd = cube.rotation * _rd;
    let ro = cube.rotation * (_ro - cube.position);

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

    let txi = transpose(cube.rotation);

    if t1.x > t1.y && t1.x > t1.z { out.normal = txi[0] * s.x; }
    else if t1.y > t1.z           { out.normal = txi[1] * s.y; }
    else                          { out.normal = txi[2] * s.z; }

    out.fraction = tN;
    out.is_intersected = true;

    return out;
}

const FAR_DISTANCE = 1000000.0;
const COUNT = 12;

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>) -> IntersectInfo {
    var out: IntersectInfo;
    var min_dist = FAR_DISTANCE;

    var sphs = array<Cube, COUNT>(
        Cube(
            1.0,
            Material(
                vec3<f32>(1.00000, 0.85882, 0.38824),
                vec3<f32>(1.0),
                0.8,
                0.3
            ),
            vec3<f32>(0.0, 0.0, 3.0),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            4,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(0.0, -5, 6),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            4,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(0.0, 5, 6),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),

        Cube(
            2,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(2.0, 0.0, 4),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            2,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(-2.0, 0.0, 4),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),

        Cube(
            6,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(0.0, 0.0, -11.0),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            6,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(10.0, 0.0, 0.0),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        // Cube(
        //     6,
        //     Material(
        //         vec3<f32>(0.0),
        //         vec3<f32>(1.0),
        //         0.1,
        //         0.0
        //     ),
        //     vec3<f32>(-10.0, 0.0, 0.0),
        //     mat3x3<f32>(
        //         1.0, 0.0, 0.0,
        //         0.0, 1.0, 0.0,
        //         0.0, 0.0, 1.0
        //     )
        // ),

        Cube(
            6,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0, 0.0, 0.0),
                0.0,
                0.0
            ),
            vec3<f32>(0.0, 10.0, 0.0),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            6,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(0.0, 1.0, 0.0),
                0.0,
                0.0
            ),
            vec3<f32>(0.0, -10.0, 0.0),
            mat3x3<f32>(
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            )
        ),

        Cube(
            1,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(2.0, -1, -4.0),
            mat3x3<f32>(
                cos(45.0), sin(45.0), 0.0,
                -sin(45.0), cos(45.0), 0.0,
                0.0, 0.0, 1.0
            )
        ),
        Cube(
            1,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(2.0, -1, -3.0),
            mat3x3<f32>(
                cos(45.0), sin(45.0), 0.0,
                -sin(45.0), cos(45.0), 0.0,
                0.0, 0.0, 1.0
            )
        ),

        Cube(
            1,
            Material(
                vec3<f32>(0.0),
                vec3<f32>(1.0),
                0.0,
                0.0
            ),
            vec3<f32>(1, 2.5, -4.0),
            mat3x3<f32>(
                cos(-25.0), sin(-25.0), 0.0,
                -sin(-25.0), cos(-25.0), 0.0,
                0.0, 0.0, 1.0
            )
        ),
    );

    for (var i = 0; i < COUNT; i++) {
        let int = box_int(ro, rd, sphs[i]);

        if int.is_intersected && int.fraction < min_dist {
            min_dist = int.fraction;
            out.normal = int.normal;
            out.material = sphs[i].material;
        }
    }

    out.fraction = min_dist;
    out.is_intersected = min_dist != FAR_DISTANCE;

    return out;
}

const MAX_DEPTH = 7;

//! ifndef _eng_rand_wgsl
//! define _eng_rand_wgsl ""

const SEED_MAX = 100000000.0;
var<private> seed: u32 = 1;

fn _rand(_co: vec2<f32>) -> f32 {
    let co = _co * fract(f32(u_time.time) * 12.343);
    return fract(sin(dot(co.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn rand(_co: vec2<f32>) -> f32 {
    let r = _rand(sin(_co * f32(seed)));

    let seed_f = fract(_rand(cos(_co * f32(seed))) * 12.774);

    let old_min = 0.0;
    let old_max = 1.0;
    let new_min = 0.0;
    let new_max = SEED_MAX;

    let old_range = old_max - old_min;
    let new_range = new_max - new_min;

    let seed_fc = (((seed_f - old_min) * new_range) / old_range) + new_min;
    seed = u32(seed_fc);

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
//! include "eng_rand"

fn trace_ray(_ro: vec3<f32>, _rd: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
//    var L = vec3<f32>(rand_noise(uv), rand_noise(uv), rand_noise(uv)) * vec3<f32>(uv, 0.0);
    var L = vec3<f32>(0.0);
    var F = vec3<f32>(1.0);

    var ro = _ro;
    var rd = _rd;

    for (var i = 0; i < MAX_DEPTH; i++) {
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

            L += F * hit.material.emmitance;
            F *= hit.material.reflectance;
        } else {
            F = vec3<f32>(0.0);
        }
    }

    return L;
}

const PI = 3.1415926535;

fn rand_point(rand: vec2<f32>) -> vec3<f32> {
    let cos_theta = sqrt(1.0 - rand.x);
    let sin_theta = sqrt(rand.x);
    let phi = 2.0 * PI * rand.y;

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

//! endif
//! include "ray_trasing"
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
    matrix: vec4<f32>
}

struct CameraPositionUniform {
    pos: vec4<f32>
}

@group(0) @binding(0) var<uniform> u_meta_data: MetaDataUniform;
@group(1) @binding(0) var<uniform> u_time: TimeUniform;
@group(2) @binding(0) var<uniform> u_cam_pos: CameraPositionUniform;
@group(3) @binding(0) var<uniform> u_cam_rot: CameraMatrixUniform;

//! endif
//! include "eng_header"
//! ifndef _eng_vertex_wgsl
//! define _eng_vertex_wgsl ""

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = vec2<f32>(model.position.x, model.position.y);

    return out;
}

//! endif
//! include "eng_vertex"

//! define SAMPLE_COUNT "5"

fn get_ray_direction(tex_coord: vec2<f32>, view_size: vec2<f32>, fov: f32, dir: vec3<f32>, up: vec3<f32>) -> vec3<f32> {
    let tex_diff = 0.5 * vec2<f32>(1.0 - 2.0 * tex_coord.x, 2.0 * tex_coord.y - 1.0);
    let angle_diff = tex_diff * vec2<f32>(view_size.x / view_size.y, 1.0) * tan(fov * 0.5);

    let ray_dir = normalize(vec3<f32>(angle_diff, 1.0)) * mat3x3<f32>(
        cos(45.0), sin(45.0), 0.0,
        -sin(45.0), cos(45.0), 0.0,
        0.0, 0.0, 1.0
    );

    let right = normalize(cross(up, dir));
    let view_to_world = mat3x3(
        right,
        up,
        dir
    );

    return view_to_world * ray_dir;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * u_meta_data.res / u_meta_data.res.y;
    
    var ray_orig = u_cam_pos.pos.xyz;

    // let angle = f32(u_time.time) / 100.0;
    let ray_dir = normalize(vec3<f32>(1.0, uv)) * mat3x3<f32>(
        cos(u_cam_rot.matrix.x), sin(u_cam_rot.matrix.x), 0.0,
        -sin(u_cam_rot.matrix.x), cos(u_cam_rot.matrix.x), 0.0,
        0.0, 0.0, 1.0
    );

    // let ray_dir = (vec4<f32>(normalize(vec3<f32>(1.0, uv)), 1.0) * u_cam_rot.matrix).xyz;

    // let ray_dir = normalize(vec3<f32>(1.0, uv));

    var color = vec3<f32>(0.0);

for (var sample = 0; sample < 5; sample++) {
//        seed = u32(sample) * u32(u_time.time);
        let tmp_color = trace_ray(ray_orig, ray_dir, uv);
        color += tmp_color;
    }

color /= f32(5);

    return vec4<f32>(color, 1.0);
}