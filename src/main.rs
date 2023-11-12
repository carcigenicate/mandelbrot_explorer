mod ui;
mod logic;

fn main() {
    let width = 7000;
    let height = width * 2/3;

    let start = std::time::Instant::now();
    ui::image::draw_area("mandelbrot.png", 0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812, width, height, 200, 2);
    println!("Elapsed: {:?}", start.elapsed());
}
