//! include "ray_trasing"
//! include "eng_header"
//! include "eng_vertex"

//! define SAMPLE_COUNT "10"

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

    //! insert "for (var sample = 0; sample < SAMPLE_COUNT; sample++) {"
//        seed = u32(sample) * u32(u_time.time);
        let tmp_color = trace_ray(ray_orig, ray_dir, uv);
        color += tmp_color;
    }

    //! insert "color /= f32(SAMPLE_COUNT);"

    return vec4<f32>(color, 1.0);
}