//! ifndef _render_ray_casting_wgsl
//! define _render_ray_casting_wgsl ""

//! include "render_def"
//! include "uniforms"

//! define FAR_DISTANCE "1000000.0"

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>) -> IntersectInfo {
    var out: IntersectInfo;
    //! insert "var min_dist = FAR_DISTANCE;"

    for (var i = 0; i < 2; i++) {
        let int = box_int(ro, rd, b_voxels[i]);

        if int.is_intersected && int.fraction < min_dist {
            min_dist = int.fraction;
            out.normal = int.normal;
            out.material = b_voxels[i].material;
        }
    }

    out.fraction = min_dist;
    //! insert "out.is_intersected = FAR_DISTANCE != min_dist;"

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