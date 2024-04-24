use camera::Camera;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod camera;
mod color;
mod ray;
mod vectors;

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let file = File::create("gradient.ppm").expect("failed to create new output file");
    let mut buf = BufWriter::new(file);
    let cam = Camera::new(aspect_ratio, image_width);
    cam.render(&mut buf)?;
    buf.flush()
}
