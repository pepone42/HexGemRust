use gem::Gem;
use hexagrid::HexaGrid;

#[derive(Debug)]
pub struct Board {
    pub grid: HexaGrid<Gem>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        Board { grid: HexaGrid::<Gem>::new(w, h) }
    }

    pub fn randomize(&self, extra_color: bool) {
        for gem in self.grid.iter() {
            gem.set_random_color(extra_color);
        }
    }
}

#[cfg(test)]
mod test {
    use board::Board;
    #[test]
    fn create_board() {
        let b = Board::new(5,7);
        let a = b.grid.get(0);
        a.set_random_color(true);
        
    }    
    
}