use num::Complex;
use crate::logic::mandelbrot_iteration;

const DENSITY_SORTED_CHARS: &str = ".`-,:\'_;~*\"\\/^i!rl+|I=)(t<j>f1}{vx?L7z][JcTnuysYkohF4eaV3205pbqdXPZUC69K#AwHmg8E%&S$DORNGQBMW@";

fn char_for_iterations(n_iterations: u32, max_iterations: u32) -> char {
    let density_perc = n_iterations as f32 / (max_iterations + 1) as f32;
    let index = ((DENSITY_SORTED_CHARS.len()) as f32 * density_perc) as usize;
    return DENSITY_SORTED_CHARS.as_bytes()[index] as char;
}

// TODO: Test area. Takes TL,BR? Returns String?

// real == x
// imag == y
pub fn test_area(real_min: f64, real_max: f64, imag_min: f64, imag_max: f64, display_width: u32, display_height: u32, max_iterations: u32, infinity_limit: u32) -> String {
    let mut result = String::new();

    let real_length = real_max - real_min;
    let imag_length = imag_max - imag_min;

    let real_step = real_length / display_width  as f64;
    let imag_step = imag_length / display_height as f64;

    for display_y in 0..display_height {
        let imag = imag_min + (display_y as f64) * imag_step;
        for display_x in 0..display_width {
            let real = real_min + (display_x as f64) * real_step;
            let c = Complex::new(real, imag);
            let iters = mandelbrot_iteration::test_point(c, max_iterations, infinity_limit);
            result.push(char_for_iterations(iters, max_iterations));
        }
        result.push('\n');
    }

    return result;
}