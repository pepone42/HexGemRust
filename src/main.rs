extern crate rand;

use color::Color;
use gem::Gem;
use board::*;

mod color;
mod hexagrid;
mod gem;
mod board;
mod actor;





fn main() {
    let mycol: Color = Color::Cyan;

    let mut grid = hexagrid::HexaGrid::<Gem>::new(5, 7);

    println!("{}", mycol);
    println!("{}", rand::random::<Color>());
    // println!("{:#?}", grid);
    println!("{}", grid);

    // simple iter
    for i in grid.iter() {
        println!("[{}]", i);
    }
    // mut iter
    for j in grid.iter_mut() {
        j.set_random_color(true);
    }
    println!("{}", grid);
    println!("Hello world !");


    let b = Board::new(5, 7);
    let a = b.grid.get(0);
    b.randomize(true);
    println!("{}", *a.borrow());
}
