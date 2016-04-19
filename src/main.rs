mod hexgem;
use hexgem::{Color,HexaGrid};

fn main() {
    let mycol : Color = Color::Cyan;

    let grid = HexaGrid::<Color>::new(5,7);

    println!("{}", mycol);
    println!("{:#?}", grid);
    println!("{}", grid);

    println!("Hello world !");
}