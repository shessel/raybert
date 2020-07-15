use raybert::img;
use raybert::math::intersections::{Hittable, Sphere};
use raybert::math::{Rayf, Vec3f};

fn ray_color(ray: &Rayf) -> Vec3f {
    let center = Vec3f::new(0.0, 0.0, -1.0);
    let sphere = Sphere::new(center, 0.5);
    if let Some(rec) = sphere.hit(ray, 0.0, f32::MAX) {
        return (rec.normal + Vec3f::new(1.0, 1.0, 1.0)) * 0.5f32;
    }
    let unit_dir = ray.direction.get_normalized();
    let t = (unit_dir.y + 1.0) * 0.5;
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.5, 0.7, 1.0) * t
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: usize = 384;
    const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
    let mut data = [0u8; WIDTH * HEIGHT * 3];

    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0f32;

    let origin = Vec3f::new(0.0, 0.0, 0.0);
    let horizontal = Vec3f::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, -viewport_height, 0.0);
    let upper_left =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3f::new(0.0, 0.0, focal_length);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / (WIDTH - 1) as f32;
            let v = y as f32 / (HEIGHT - 1) as f32;
            let ray = Rayf::new(origin, upper_left + horizontal * u + vertical * v - origin);
            let color = ray_color(&ray);
            let i = (y * WIDTH + x) * 3;
            data[i] = (color.x * 255.99999).floor() as u8;
            data[i + 1] = (color.y * 255.99999).floor() as u8;
            data[i + 2] = (color.z * 255.99999).floor() as u8;
        }
    }

    img::write_png(&data, WIDTH as u32, HEIGHT as u32, r"test.png");
}
