//! ifndef _eng_rand_wgsl
//! define _eng_rand_wgsl ""

//! include "eng_header"

var<private> seed: f32 = 0.0;

fn _rand(_co: vec2<f32>) -> f32 {
    let co = _co * fract(f32(u_time.time) * 12.343);
    //let co = _co;
    return fract(sin(dot(co.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn rand(_co: vec2<f32>) -> f32 {
    let r = _rand(sin(_co * f32(seed)));

    let seed_f = fract(_rand(cos(_co * f32(seed))) * 12.774);
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