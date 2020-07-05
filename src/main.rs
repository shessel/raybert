use raybert::{math, img};

fn main() {
    let v1 = math::Vec3::<f32> {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let v2 = math::Vec3::<f32> {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let v3 = v1 + v2;
    println!("{:?}", v3);
    let data = [255, 0, 0, 255, 0, 0, 0, 255];
    img::write_png(&data, 1, 2, r"test.png");
}
