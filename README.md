# Rust Mandelbrot Explorer Re-Write

This is my attempt to re-write my old [Mandelbrot Set Explorer](https://github.com/carcigenicate/mandelbrot) written in Clojure.

Image generation works, but I'm having difficulties with the UI.

## Use

To have it generate an image, run

    cargo run
	
The location in the complex plane can be set in `main.rs`, and the exact coloring can be set in `ui/image.rs` (`color_f`). I didn't simplify it because I didn't intend to leave it in this state for very long. 