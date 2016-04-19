extern crate rand;

mod hexgem;
use hexgem::{Color, HexaGrid};

fn main() {
    let mycol: Color = Color::Cyan;

    let mut grid = HexaGrid::<Color>::new(5, 7);

    println!("{}", mycol);
    println!("{}", rand::random::<Color>());
    // println!("{:#?}", grid);
    println!("{}", grid);
    grid.randomize();
    println!("{}", grid);

    println!("Hello world !");
}