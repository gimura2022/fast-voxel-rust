//! include "std"

//! define SAMPLE_COUNT "10"

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * u_meta_data.res / u_meta_data.res.y;
    
    var ray_orig = u_camera.pos.xyz;
    let ray_dir = (normalize(vec3<f32>(1.0, uv)) * u_camera.matrix).xyz;

    var color = vec3<f32>(0.0);

    //! insert "for (var sample = 0; sample < SAMPLE_COUNT; sample++) {"
        seed = (f32(sample) + uv * 18.23189).x;
        let tmp_color = trace_ray(ray_orig, ray_dir, uv + f32(sample) * 23.00231);
        color += tmp_color;
    }

    //! insert "color /= f32(SAMPLE_COUNT);"

    return vec4<f32>(color, 1.0);
}