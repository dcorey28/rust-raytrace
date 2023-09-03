use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod vectors;

const PPM_IDENTIFIER: &str = "P3";

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("gradient.ppm").expect("failed to create new gradient file");
    let mut buf = BufWriter::new(file);

    render_gradient(&mut buf, image_width, image_height)?;

    buf.flush()
}

fn render_gradient(mut writer: impl Write, width: i32, height: i32) -> std::io::Result<()> {
    write_ppm_headers(&mut writer, width, height)?;
    calc_gradient(&mut writer, width, height)
}

fn write_ppm_headers(mut writer: impl Write, width: i32, height: i32) -> std::io::Result<()> {
    let max_saturation = u8::MAX;

    writeln!(writer, "{PPM_IDENTIFIER}")?;
    writeln!(writer, "{width} {height}")?;
    writeln!(writer, "{max_saturation}")
}

fn calc_gradient(mut writer: impl Write, width: i32, height: i32) -> std::io::Result<()> {
    let progress = ProgressBar::new((width * height) as u64);
    progress.set_style(
        ProgressStyle::with_template(
            "{msg} {wide_bar:.Magenta}{spinner} {percent}% eta:[{eta_precise}]",
        )
        .unwrap(),
    );
    progress.set_message("rendering");

    for row in 0..height {
        for col in 0..width {
            let pixel = calc_gradient_pixel(height, width, row, col);
            writeln!(writer, "{} {} {}", pixel.0, pixel.1, pixel.2)?;
            progress.inc(1);
        }
    }

    progress.finish_with_message("rendering finished");

    Ok(())
}

fn calc_gradient_pixel(height: i32, width: i32, row: i32, col: i32) -> (u8, u8, u8) {
    let max_saturation = u8::MAX;

    let red = col as f32 / (width - 1) as f32;
    let green = row as f32 / (height - 1) as f32;
    let blue = 0;

    (
        (red * max_saturation as f32) as u8,
        (green * max_saturation as f32) as u8,
        blue,
    )
}
