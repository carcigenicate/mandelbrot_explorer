use image::{ImageBuffer, Rgb};
use num::{clamp, Num};

use crate::logic::mandelbrot_iteration;

fn wrap<N: Num + Copy>(n: N, min_n: N, max_n: N) -> N {
    let limit = max_n - min_n;
    return (n % limit) + min_n;
}



fn color_f(real: f64, imag: f64, iters: u32) -> Rgb<u8> {
    let f_iters = iters as f64;
    let red = clamp(real * f_iters * 6.0, 0.0, 255.0);
    let green = clamp(imag * f_iters * 3.0, 0.0, 255.0);
    let blue = clamp(f_iters * f_iters * 0.01, 0.0, 255.0);

    return Rgb([red as u8, green as u8, blue as u8]);
}

pub fn draw_area(file_path: &str, real_min: f64, real_max: f64, imag_min: f64, imag_max: f64, display_width: u32, display_height: u32, max_iterations: u32, infinity_limit: u32) -> () {
    let mut buffer = ImageBuffer::new(display_width, display_height);

    let total_pixels = display_width * display_height;
    let report_every = total_pixels as u32 / 10;

    let mut current_pixel = 0;
    mandelbrot_iteration::test_area(real_min, real_max, imag_min, imag_max, display_width, display_height, max_iterations, infinity_limit, |real, imag, x, y, iters | {
        let pixel = buffer.get_pixel_mut(x, y);
        *pixel = color_f(real, imag, iters);

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
