use num::complex::Complex;

pub type ComplexPoint = Complex<f64>;

fn mandelbrot_iteration(initial: ComplexPoint, current: ComplexPoint) -> ComplexPoint {
    let current_sq = current * current;
    return Complex {
        re: current_sq.re + initial.re,
        im: current_sq.im + initial.im,
    };
}

fn is_under_limit(current: ComplexPoint, infinity_limit: u32) -> bool {
    let real_sq = current.re * current.re;
    let imag_sq = current.im * current.im;

    return (real_sq + imag_sq) <= (infinity_limit * infinity_limit) as f64;
}

pub fn test_point(initial: ComplexPoint, max_iterations: u32, infinity_limit: u32) -> u32 {
    let mut current = initial;

    let mut i = 0;
    while i < max_iterations {
        if is_under_limit(current, infinity_limit) {
            current = mandelbrot_iteration(initial, current);
        } else {
            break;
        }

        i += 1;
    }

    return i;
}

/// Callback arguments: real, imag, display_x, display_y, iters
pub fn test_area
<F: FnMut(f64, f64, u32, u32, u32) -> ()>
(
    real_min: f64,
    real_max: f64,
    imag_min: f64,
    imag_max: f64,
    display_width: u32,
    display_height: u32,
    max_iterations: u32,
    infinity_limit: u32,
    mut point_callback: F,
) -> () {
    let real_length = real_max - real_min;
    let imag_length = imag_max - imag_min;

    let real_step = real_length / display_width  as f64;
    let imag_step = imag_length / display_height as f64;

    for display_y in 0..display_height {
        let imag = imag_min + (display_y as f64) * imag_step;
        for display_x in 0..display_width {
            let real = real_min + (display_x as f64) * real_step;
            let c = Complex::new(real, imag);
            let iters = test_point(c, max_iterations, infinity_limit);
            point_callback(real, imag, display_x, display_y, iters);
        }
    }
}