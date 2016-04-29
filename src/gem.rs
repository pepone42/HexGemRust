use std::fmt;
use std::cell::Cell;
use rand;
use color::Color;
use actor::Actor;

enum State {
    Idle,
    Swap,
    Destroy,
}

#[derive(Debug,Default,Clone)]
pub struct Gem {
    color: Cell<Color>,
}

impl Gem {
    pub fn new() -> Self {
        Gem { color: Cell::new(Color::default()) }
    }
    pub fn new_random(extra_color: bool) -> Self {
        Gem { color: Cell::new(Color::random(&mut rand::thread_rng(), extra_color)) }
    }
    pub fn set_color(&self, color: Color) {
        self.color.set(color);
    }
    pub fn get_color(&self) -> Color {
        self.color.get()
    }
    pub fn set_random_color(&self, extra_color: bool) {
        //self.color = Color::random(&mut rand::thread_rng(), extra_color);
        self.set_color(Color::random(&mut rand::thread_rng(), extra_color))
    }
}

impl Actor for Gem {
    fn update(&mut self) {
        println!("Update {}", &self);
    }
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_color())
    }
}
