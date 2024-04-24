use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    color::Color,
    ray::Ray,
    vectors::{Point, Vec3},
};
use std::{cmp::max, io::Write};

const PPM_IDENTIFIER: &str = "P3";
const COLOR_SATURATION: f64 = u8::MAX as f64;

const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

const BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

pub struct Camera {
    center: Point<f64>,
    image: Image,
    viewport: Viewport,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = calc_image_height(aspect_ratio, image_width);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        // We don't use aspect_ratio when computing viewport_width because
        // aspect_ratio is the ideal ratio and may not be the actual ratio
        // between image_width and image_height.
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let camera_center = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v = Vec3 {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };

        let pixel_delta_horizontal = viewport_u / image_width as f64;
        let pixel_delta_vertical = viewport_v / image_height as f64;

        let viewport_origin = camera_center
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;

        Self {
            center: camera_center,
            image: Image {
                width: image_width,
                height: image_height,
            },
            viewport: Viewport {
                center: viewport_origin + (pixel_delta_horizontal + pixel_delta_vertical) * 0.5,
                delta_horizontal: pixel_delta_horizontal,
                delta_vertical: pixel_delta_vertical,
            },
        }
    }

    pub fn render(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let progress = new_progress_bar((self.image.height * self.image.width) as u64);
        progress.set_message("rendering");

        self.write_headers(writer)?;
        self.render_pixels(writer, &progress)?;

        progress.finish_with_message("rendering finished");

        Ok(())
    }

    fn write_headers(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(writer, "{PPM_IDENTIFIER}")?;
        writeln!(writer, "{} {}", self.image.width, self.image.height)?;
        writeln!(writer, "{}", COLOR_SATURATION as u8)
    }

    fn render_pixels(
        &self,
        writer: &mut impl Write,
        progress: &ProgressBar,
    ) -> std::io::Result<()> {
        for row in 0..self.image.height {
            for col in 0..self.image.width {
                let pixel_center = self.viewport.pixel_center(row, col);
                let ray = Ray {
                    orig: self.center,
                    dir: pixel_center - self.center,
                };
                let pixel = ray_color(ray);
                pixel.write(writer, COLOR_SATURATION)?;
                progress.inc(1);
            }
        }

        Ok(())
    }
}

pub struct Viewport {
    pub center: Vec3<f64>,
    pub delta_horizontal: Vec3<f64>,
    pub delta_vertical: Vec3<f64>,
}

impl Viewport {
    pub fn pixel_center(&self, row: i32, col: i32) -> Point<f64> {
        self.center + self.delta_horizontal * col as f64 + self.delta_vertical * row as f64
    }
}

pub struct Image {
    pub width: i32,
    pub height: i32,
}

fn calc_image_height(aspect_ratio: f64, image_width: i32) -> i32 {
    // ensure image height is at least 1 pixel
    max((image_width as f64 / aspect_ratio) as i32, 1)
}

fn new_progress_bar(pixels: u64) -> ProgressBar {
    let progress = ProgressBar::new(pixels);
    progress.set_style(
        ProgressStyle::with_template(
            "{msg} {wide_bar:.Magenta}{spinner} {percent}% eta:[{eta_precise}]",
        )
        .unwrap(),
    );

    progress
}

fn ray_color(ray: Ray<f64>) -> Color {
    let unit_dir = ray.dir.unit();
    let a = (unit_dir.y + 1.0) * 0.5;

    // Return a gradient between white and blue
    WHITE * (1.0 - a) + BLUE * a
}
