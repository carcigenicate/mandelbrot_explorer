use num::Complex;
use crate::logic::mandelbrot_iteration;

const DENSITY_SORTED_CHARS: &str = ".`-,:\'_;~*\"\\/^i!rl+|I=)(t<j>f1}{vx?L7z][JcTnuysYkohF4eaV3205pbqdXPZUC69K#AwHmg8E%&S$DORNGQBMW@";

fn char_for_iterations(n_iterations: u32, max_iterations: u32) -> char {
    let density_perc = n_iterations as f32 / (max_iterations + 1) as f32;
    let index = ((DENSITY_SORTED_CHARS.len()) as f32 * density_perc) as usize;
    return DENSITY_SORTED_CHARS.as_bytes()[index] as char;
}

pub fn test_area_to_text(real_min: f64, real_max: f64, imag_min: f64, imag_max: f64, display_width: u32, display_height: u32, max_iterations: u32, infinity_limit: u32) -> String {
    let mut result = String::new();

    mandelbrot_iteration::test_area(real_min, real_max, imag_min, imag_max, display_width, display_height, max_iterations, infinity_limit, |real, imag, x, y, iters| {
        result.push(char_for_iterations(iters, max_iterations));

        if x >= display_width - 1 {
            result.push('\n');
        }
    });

    return result;
}