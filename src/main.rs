extern crate rand;

mod color;
mod hexagrid;
use color::Color;

fn main() {
    let mycol: Color = Color::Cyan;

    let mut grid = hexagrid::HexaGrid::<Color>::new(5, 7);

    println!("{}", mycol);
    println!("{}", rand::random::<Color>());
    // println!("{:#?}", grid);
    println!("{}", grid);
    grid.randomize();
    println!("{}", grid);

    println!("Hello world !");
}