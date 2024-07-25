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