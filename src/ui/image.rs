use std::cmp::min;
use image::{ImageBuffer, Rgb};

use crate::logic::mandelbrot_iteration;

fn color_f(iters: u32) -> Rgb<u8> {
    let red = iters * 1;
    let green = iters * 2;
    let blue = iters * 3;

    return Rgb([
        min(red, 255) as u8,
        min(green, 255) as u8,
        min(blue, 255) as u8,
    ]);
}

pub fn draw_area(file_path: &str, real_min: f64, real_max: f64, imag_min: f64, imag_max: f64, display_width: u32, display_height: u32, max_iterations: u32, infinity_limit: u32) -> () {
    let mut buffer = ImageBuffer::new(display_width, display_height);

    let total_pixels = (display_width * display_height);
    let report_every = total_pixels as u32 / 100;

    let mut current_pixel = 0;
    mandelbrot_iteration::test_area(real_min, real_max, imag_min, imag_max, display_width, display_height, max_iterations, infinity_limit, | real, image, x, y, iters | {
        let pixel = buffer.get_pixel_mut(x, y);
        *pixel = color_f(iters);

        current_pixel += 1;
        if current_pixel % report_every == 0 {
            let perc = current_pixel as f32 / total_pixels as f32;
            println!("Progress: {}/{} ({}%)", current_pixel, total_pixels, perc * 100.0);
        }
    });

    match buffer.save(file_path) {
        Ok(_) => println!("Saved successfully"),
        Err(err) => println!("Error: {}", err),
    };
}
