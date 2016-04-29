extern crate rand;

use board::*;

mod color;
mod hexagrid;
mod gem;
mod board;
mod actor;





fn main() {
    let b = Board::new(5, 7);
    let a = b.grid.get(0);
    println!("[{}]", a.get_color());
    b.randomize(true);
    println!("[{}]", a.get_color());
}
