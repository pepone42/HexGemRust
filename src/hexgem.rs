use std::fmt;
use rand::{Rand,Rng,random};

#[derive(Clone,Debug)]
pub enum Color {
    Empty,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Brown,
    Cyan
}

impl Color {
    fn get_string_representation(&self) -> &'static str {
        match *self {
            Color::Empty => "  ",
            Color::Red   => "Re",
            Color::Green => "Gr",
            Color::Blue  => "Bl",
            Color::Yellow => "Ye",
            Color::Purple => "Pu",
            Color::Brown => "Br",
            Color::Cyan => "Cy"
        }
    }
}

impl Default for Color {
    fn default() -> Color { Color::Empty }
}
// TODO: There must be a more elegant way to get a random enum
impl Rand for Color {
    fn rand<R : Rng>(rng: &mut R) -> Self {
        let a = rng.gen_range(1,7);
        match a {
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Blue,
            4 => Color::Yellow,
            5 => Color::Purple,
            6 => Color::Brown,
            7 => Color::Cyan,
            _ => unreachable!(),
        }
    }
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_string_representation())
    }
}


#[derive(Debug)]
pub struct HexaGrid<T> {
    width : usize,
    heigth : usize,
    data: Box<[T]>
}

impl<T: Clone + Default + Rand> HexaGrid<T> {
    pub fn new(w: usize,h: usize) -> HexaGrid<T> {
        let flat_size = h * w + (h / 2);
        HexaGrid::<T> { width : w, heigth : h, data : vec![<T>::default(); flat_size].into_boxed_slice()}
    }
}

impl<T: fmt::Display> fmt::Display for HexaGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res : String = "".to_string();
        for i in 0..self.heigth {
            let w = if (i & 1) == 0 {
                res = format!("{}  ",res);
                self.width
            } else {self.width + 1};
            for j in 0..w {
                res = format!("{}[{}]",res ,self.data[j]);
            }
            res = format!("{}\n",res);
        }
        write!(f,"{}",res)
    }
}
