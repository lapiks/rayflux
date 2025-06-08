struct VertexOutput {
    @builtin(position) position : vec4<f32>,
    @location(0) fragUV : vec2<f32>,
};

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx : u32) -> VertexOutput {
    var out: VertexOutput;

    let positions = array<vec2<f32>, 3>(
        vec2(-1.0, -1.0), // bottom-left
        vec2(3.0, -1.0),  // bottom-right (offscreen)
        vec2(-1.0, 3.0)   // top-left (offscreen)
    );

    let pos = positions[vertex_idx];
    out.position = vec4<f32>(pos, 0.0, 1.0);

    // Convert from NDC [-1,1] to UV [0,1]
    out.fragUV = (pos + vec2(1.0)) * 0.5;

    return out;
}

@group(0) @binding(0) var t_color : texture_2d<f32>;
@group(0) @binding(1) var t_sampler : sampler;

@fragment
fn fs_main(@location(0) fragUV : vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(t_color, t_sampler, fragUV);
}