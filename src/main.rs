mod ui;
mod logic;

// "Classic" [0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812]

fn main() {
    let width = 200;
    let result = ui::text::test_area_to_text(0.3539265474695936, 0.3607889277625950, 0.3531315391015801, 0.3599939193945812, width, width * 2/3, 200, 2);
    println!("{}", result);
}
