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
