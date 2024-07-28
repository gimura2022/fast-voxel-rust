//! ifndef _render_ray_casting_wgsl
//! define _render_ray_casting_wgsl ""

//! include "render_def"
//! include "uniforms"

//! define FAR_DISTANCE "1000000.0"

fn cast_ray(ro: vec3<f32>, rd: vec3<f32>) -> IntersectInfo {
    var out: IntersectInfo;
    //! insert "var min_dist = FAR_DISTANCE;"

    var box = b_voxels[0];

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
            //! insert "var child_min_dist = FAR_DISTANCE;"
            var box_out = box;

            for (var i = 0; i < 8; i++) {
                let child_box = get_voxel(u32(box.childs[i]));
                let child_int = box_int(ro, rd, child_box);

                if child_int.is_intersected && child_int.fraction < child_min_dist {
                    child_min_dist = child_int.fraction;
                    box_out = child_box;
                }
            }
            
            box = box_out;

            //! insert "if child_min_dist == FAR_DISTANCE {"
                out.is_intersected = false;
                break;
            }

            continue;
        }
    }

    return out;

    // for (var i = 0; i < 1; i++) {

    //     if int.is_intersected && int.fraction < min_dist {
    //         min_dist = int.fraction;
    //         out.normal = int.normal;
    //         out.material = b_voxels[i].material;
    //     }
    // }

    // out.fraction = min_dist;
    // //! insert "out.is_intersected = FAR_DISTANCE != min_dist;"

    // return out;
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