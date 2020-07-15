use super::{dot, Rayf, Vec3f};

pub fn ray_sphere(center: &Vec3f, radius: f32, ray: &Rayf) -> bool {
    let origin_to_center = *center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0f32 * dot(origin_to_center, ray.direction);
    let c = dot(origin_to_center, origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0f32 * a * c;
    discriminant > 0.0
}
