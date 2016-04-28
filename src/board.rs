use std::cell::RefCell; 
use gem::Gem;
use hexagrid::HexaGrid;

#[derive(Debug)]
pub struct Board {
    pub grid: HexaGrid<RefCell<Gem>>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        Board { grid: HexaGrid::<RefCell<Gem>>::new(w, h) }
    }

    pub fn randomize(&self, extra_color: bool) {
        for gem in self.grid.iter() {
            gem.borrow_mut().set_random_color(extra_color);
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
        a.borrow_mut().set_random_color(true);
        
    }    
    
}