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