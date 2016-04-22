extern crate rand;

mod color;
mod hexagrid;
mod gem;
mod board;
mod actor;



use color::Color;
use gem::Gem;

fn main() {
    let mycol: Color = Color::Cyan;

    let mut grid = hexagrid::HexaGrid::<Gem>::new(5, 7);

    println!("{}", mycol);
    println!("{}", rand::random::<Color>());
    // println!("{:#?}", grid);
    println!("{}", grid);

    for i in grid.iter() {
        println!("{}", i);
    }

    println!("Hello world !");
}