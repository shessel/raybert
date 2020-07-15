use super::{dot, Rayf, Vec3f};

pub struct HitRecord {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3f, normal: Vec3f, t: f32, ray: &Rayf) -> HitRecord {
        let front_face = dot(normal, ray.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Rayf, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Rayf, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;

        let a = ray.direction.len_squared();
        let half_b = dot(origin_to_center, ray.direction);
        let c = origin_to_center.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;

            if t > t_max || t < t_min {
                t = (-half_b + root) / a;
            }

            if t <= t_max && t >= t_min {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                let rec = HitRecord::new(point, normal, t, ray);
                return Some(rec);
            }
        }

        None
    }
}
