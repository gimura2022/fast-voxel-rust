//! ifndef _rand_wgsl
//! define _rand_wgsl ""

//! include "std" "header.wgsl"

var<private> rand_state: vec4<u32>;

fn taus_step(z: u32, s1: u32, s2: u32, s3: u32, m: u32) -> u32 {
    let b = (((z << s1) ^ z) >> s2);
    return (((z & m) << s3) ^ b);
}

fn lcg_step(z: u32, a: u32, c: u32) -> u32 {
    return (a * z + c);
}

fn rand() -> f32 {
    rand_state.x = taus_step(rand_state.x, u32(13), u32(19), u32(12), u32(4294967294));
    rand_state.y = taus_step(rand_state.y, u32(2), u32(25), u32(4), u32(4294967288));
    rand_state.z = taus_step(rand_state.z, u32(3), u32(11), u32(17), u32(4294967280));
    rand_state.w = lcg_step(rand_state.w, u32(1664525), u32(1013904223));

    return 2.3283064365387e-10 * f32((rand_state.x ^ rand_state.y ^ rand_state.z ^ rand_state.w));
}

fn rand2() -> vec2<f32> {
    return vec2<f32>(rand(), rand());
}

fn rand3() -> vec3<f32> {
    return vec3<f32>(rand(), rand(), rand());
}

fn rand4() -> vec4<f32> {
    return vec4<f32>(rand(), rand(), rand(), rand());
}

//! endif