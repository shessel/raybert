use super::{dot, Rayf, Vec3f};

pub fn ray_sphere(center: &Vec3f, radius: f32, ray: &Rayf) -> f32 {
    let origin_to_center = ray.origin - *center;
    let a = ray.direction.len_squared();
    let half_b = dot(origin_to_center, ray.direction);
    let c = origin_to_center.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
