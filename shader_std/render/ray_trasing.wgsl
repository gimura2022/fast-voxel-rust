//! ifndef _render_ray_trasing_wgsl
//! define _render_ray_trasing_wgsl ""

//! define MAX_DEPTH "4"

//! include "std" "render_def.wgsl"
//! include "std" "ray_casting.wgsl"
//! include "std" "math.wgsl"
//! include "std" "rand.wgsl"

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

fn trace_ray(_ro: vec3<f32>, _rd: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
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

            L += F * hit.material.emmitance.xyz;
            F *= hit.material.reflectance.xyz;
        } else {
            F = vec3<f32>(0.0);
        }
    }

    return L;
}

//! endif