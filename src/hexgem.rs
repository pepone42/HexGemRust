use std::fmt;
// use rand;
use rand::{Rand, Rng, random, thread_rng};

#[derive(Clone,Debug)]
pub enum Color {
    Empty,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Brown,
    Cyan,
}

const ALL_COLORS: [Color; 8] = [Color::Empty,
                                Color::Red,
                                Color::Green,
                                Color::Blue,
                                Color::Yellow,
                                Color::Purple,
                                Color::Brown,
                                Color::Cyan];

impl Color {
    fn get_string_representation(&self) -> &'static str {
        match *self {
            Color::Empty => "  ",
            Color::Red => "Re",
            Color::Green => "Gr",
            Color::Blue => "Bl",
            Color::Yellow => "Ye",
            Color::Purple => "Pu",
            Color::Brown => "Br",
            Color::Cyan => "Cy",
        }
    }
    fn random(extraColor: bool) -> Color {
        let maxcol = if extraColor == true {
            ALL_COLORS.len()
        } else {
            ALL_COLORS.len() - 1
        };
        ALL_COLORS[thread_rng().gen_range(1, maxcol)].clone()
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::Empty
    }
}
// TODO: There must be a more elegant way to get a random enum
impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let a = rng.gen_range(1, 8);
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
    width: usize,
    heigth: usize,
    data: Box<[T]>,
}

impl<T: Clone + Default + Rand> HexaGrid<T> {
    pub fn new(w: usize, h: usize) -> HexaGrid<T> {
        let flat_size = h * w + (h / 2);
        HexaGrid::<T> {
            width: w,
            heigth: h,
            data: vec![<T>::default(); flat_size].into_boxed_slice(),
        }
    }
    pub fn randomize(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = random::<T>();
        }
    }
}

impl<T: fmt::Display> fmt::Display for HexaGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Approxiamte capacity : (4 * (w+1)*h
        let mut res: String = String::with_capacity("[  ]".len() * (self.width + 1) * self.heigth);
        let mut index = 0;
        for i in 0..self.heigth {
            let w = if (i & 1) == 0 {
                res.push_str("  ");
                self.width
            } else {
                self.width + 1
            };
            for _ in 0..w {
                res.push_str(format!("[{}]", self.data[index]).as_str());
                index += 1;
            }
            res.push_str("\n");
        }
        f.write_str(res.as_str())
    }
}
