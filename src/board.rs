use gem::Gem;
use hexagrid::HexaGrid;

#[derive(Debug)]
pub struct Board {
    grid: HexaGrid<Gem>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        Board { grid: HexaGrid::<Gem>::new(w, h) }
    }

    pub fn randomize(&mut self, extra_color: bool) {}
}
