use std::fmt;
use rand::{Rand, Rng};

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
    fn random<R: Rng>(rng: &mut R, extra_color: bool) -> Color {
        let maxcol = if extra_color == true {
            ALL_COLORS.len()
        } else {
            ALL_COLORS.len() - 1
        };
        ALL_COLORS[rng.gen_range(1, maxcol)].clone()
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
        Color::random(rng, true)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_string_representation())
    }
}
