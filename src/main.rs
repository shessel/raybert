use raybert::img;
use raybert::math::{Rayf, Vec3f};

fn ray_color(ray: &Rayf) -> Vec3f {
    let unit_dir = ray.direction.get_normalized();
    let t = (unit_dir.y + 1.0) * 0.5;
    Vec3f::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3f::new(0.5, 0.7, 1.0) * t
}

fn main() {
    const aspect_ratio: f32 = 16.0 / 9.0;
    const width: usize = 384;
    const height: usize = (width as f32 / aspect_ratio) as usize;
    let mut data = [0u8; width * height * 3];

    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0f32;

    let origin = Vec3f::new(0.0, 0.0, 0.0);
    let horizontal = Vec3f::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3f::new(0.0, -viewport_height, 0.0);
    let upper_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3f::new(0.0, 0.0, focal_length);

    for y in 0..height {
        for x in 0.. width {
            let u = x as f32 / (width - 1) as f32;
            let v = y as f32 / (height - 1) as f32;
            let ray = Rayf::new(origin, upper_left + horizontal * u + vertical * v - origin);
            let color = ray_color(&ray);
            let i = (y * width + x) * 3;
            data[i] = (color.x * 255.99999).floor() as u8;
            data[i + 1] = (color.y * 255.99999).floor() as u8;
            data[i + 2] = (color.z * 255.99999).floor() as u8;
        }
    }

    img::write_png(&data, width as u32, height as u32, r"test.png");
}
