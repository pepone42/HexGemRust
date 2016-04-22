use std::fmt;
use rand;
use rand::{Rand, Rng};
use color::Color;
use actor::Actor;

enum State {
    Idle,
    Swap,
    Destroy,
}

#[derive(Debug,Default,Clone)]
pub struct Gem {
    color: Color,
}

impl Gem {
    pub fn new() -> Self {
        Gem { color: Color::default() }
    }
    pub fn new_random(extra_color: bool) -> Self {
        Gem { color: Color::random(&mut rand::thread_rng(), extra_color) }
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn set_random_color(&mut self, extra_color: bool) {
        self.color = Color::random(&mut rand::thread_rng(), extra_color);
    }
}

impl Actor for Gem {
    fn update(&mut self) {
        println!("Update {}", &self);
    }
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}
