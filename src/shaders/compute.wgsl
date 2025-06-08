struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

@group(0) @binding(0)
var out_image : texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(out_image);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    let color = vec4<f32>(f32(id.x) / f32(dims.x), f32(id.y) / f32(dims.y), 0.5, 1.0);
    textureStore(out_image, vec2<i32>(id.xy), color);
}