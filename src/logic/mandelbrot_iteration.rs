use num::complex::Complex;

// export function mapRange(n: number, sourceMin: number, sourceMax: number, targetMin: number, targetMax: number): number {
// const sourceRatio = (n - sourceMin) / (sourceMax - sourceMin);
//
// return (sourceRatio * (targetMax - targetMin)) + targetMin;
// }

// import * as math from 'mathjs';
// import {Complex} from 'mathjs';
//
// // Port of my old C code
// // https://codereview.stackexchange.com/questions/217574/ascii-mandelbrot-set-image-producer
//
//
// export function testPoint(initial: Complex, maxIterations: number, infinityLimit: number = 2): number {
//   let current = initial;
//
//   let i;
//   for (i = 0; i < maxIterations; i++) {
//     if (isUnderLimit(current, infinityLimit)) {
//       current = mandelbrotIteration(initial, current);
//     } else {
//       break;
//     }
//   }
//
//   return i;
// }

pub type ComplexPoint = Complex<i64>;

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

    return (real_sq + imag_sq) <= (infinity_limit * infinity_limit) as i64;
}

// TODO: u32 for max_iterations and the return can be generic technically
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
