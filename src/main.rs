const MAX_SATURATION: u8 = 255;
const PPM_IDENTIFIER: &str = "P3";

fn main() {
    let image_width = 256;
    let image_height = 256;

    render_gradient(image_width, image_height);
}

fn render_gradient(width: i32, height: i32) {
    print_image_headers(width, height, MAX_SATURATION);
    print_gradient(width, height, MAX_SATURATION)
}

fn print_image_headers(width: i32, height: i32, saturation: u8) {
    println!("{PPM_IDENTIFIER}");
    println!("{width} {height}");
    println!("{saturation}");
}

fn print_gradient(width: i32, height: i32, max_saturation: u8) {
    for row in 0..height {
        for col in 0..width {
            let pixel = calc_gradient_pixel(height, width, row, col, max_saturation);
            println!("{} {} {}", pixel.0, pixel.1, pixel.2);
        }
    }
}

fn calc_gradient_pixel(height: i32, width: i32, row: i32, col: i32, max_saturation: u8) -> (u8, u8, u8) {
    let red = col as f32 / (width - 1) as f32;
    let green = row as f32 / (height - 1) as f32;
    let blue = 0;

    (
        (red * max_saturation as f32) as u8,
        (green * max_saturation as f32) as u8,
        blue,
    )
}
