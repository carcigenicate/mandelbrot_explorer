mod ui;
mod logic;

// Zoomed Out [-2, 2, -2, 2]
// "Classic" [0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812]
// Purple Swirl [0.2720061643835617 0.2722061643835617 -0.0056806685236769 -0.0054806685236769]

fn main() {
    // let width = 200;
    // //let result = ui::text::test_area_to_text(0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812, width, width / 2, 200, 2);
    // let result = ui::text::draw_area(0.2720061643835617, 0.2722061643835617, -0.0056806685236769, -0.0054806685236769, width, width / 2, 200, 2);
    // //let result = ui::text::test_area_to_text(-2.0, 2.0, -2.0, 2.0, width, width / 2, 200, 2);
    // println!("{}", result);

    let width = 30000;
    let height = width * 2/3;
    ui::image::draw_area("mandelbrot.png",0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812, width, height, 200, 2);
}
