struct Camera {
    inv_view_proj: mat4x4<f32>,
    position: vec3<f32>,
    _pad: f32,
};

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> f32 {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        return -1.0;
    }

    return (-b - sqrt(discriminant)) / (2.0 * a);
}

@group(0) @binding(0)
var out_image : texture_storage_2d<rgba8unorm, write>;

@group(1) @binding(0) 
var<uniform> camera: Camera;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(out_image);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    // Screen point to [-1, 1]
    let uv = (vec2<f32>(id.xy) / vec2<f32>(dims)) * 2.0 - vec2<f32>(1.0);
    let clip = vec4<f32>(uv, -1.0, 1.0);

    // Clip to world space 
    let world_pos = camera.inv_view_proj * clip;
    // World space to NDC
    let world_pos_ndc = world_pos.xyz / world_pos.w;

    // Create a ray
    let dir = normalize(world_pos_ndc - camera.position);
    let ray = Ray(camera.position, dir);

    // Sphere
    let sphere = Sphere(vec3<f32>(0.0, 0.0, 0.0), 1.0);

    let t = intersect_sphere(ray, sphere);

    var color: vec4<f32>;

    if (t > 0.0) {
        let hit_point = ray_at(ray, t);
        let normal = normalize(hit_point - sphere.center);
        color = vec4<f32>(normal * 0.5 + 0.5, 1.0); // RGB encode normal
    } else {
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0); // background
    }

    textureStore(out_image, vec2<i32>(id.xy), color);
}