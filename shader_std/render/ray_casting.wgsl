//! ifndef _render_ray_casting_wgsl
//! define _render_ray_casting_wgsl ""

//! include "std" "render_def.wgsl"
//! include "std" "uniforms.wgsl"

//! define FAR_DISTANCE "1000000.0"

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>) -> IntersectInfo {
    var out: IntersectInfo;
    var min_dist = FAR_DISTANCE;

    var box = b_voxels[8];

    while true {
        let int = box_int(ro, rd, box);

        if !int.is_intersected {
            out = int;
            break;
        }

        if box.is_leaf >= 1.0 {
            out = int;
            out.material = box.material;

            break;
        } else {
            var child_min_dist = FAR_DISTANCE;
            var box_out = box;

            for (var i = 0; i < 8; i++) {
                let child_box = get_voxel(u32(box.childs[i]));
                let child_int = box_int(ro, rd, child_box);

                if child_int.is_intersected && child_int.fraction < child_min_dist && child_box.is_none != 1.0 {
                    child_min_dist = child_int.fraction;
                    box_out = child_box;
                }
            }
            
            box = box_out;

            if child_min_dist == FAR_DISTANCE {
                out.is_intersected = false;
                break;
            }

            continue;
        }

        break;
    }

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