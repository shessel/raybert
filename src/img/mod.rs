use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::{BitDepth, ColorType, Encoder};

pub fn write_png(data: &[u8], width: u32, height: u32, path: &str) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = Encoder::new(buf_writer, width, height);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap(); // Save
}
