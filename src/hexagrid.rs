use std::fmt;
use std::slice;

#[derive(Debug)]
/// An Hexagonal Grid
pub struct HexaGrid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct HexaGridIter<'a, T: 'a> {
    grid: &'a HexaGrid<T>,
    index: usize,
}

impl<'a, T> Iterator for HexaGridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.index >= self.grid.data.len() {
            None
        } else {
            self.index += 1;
            Some(&self.grid.data[self.index - 1])
        }
    }
}

/// Ring iterator
pub struct HexaGridRingInter<'a, T: 'a> {
    grid: &'a HexaGrid<T>,
    index: usize,
    position: usize,
}

// pub struct HexaGridMutIter<'a, T: 'a> {
//     data: slice::IterMut<'a, T>,
// }

// impl<'a, T> Iterator for HexaGridMutIter<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.data.next().map(|item| item)
//     }
// }

impl<'a, T: 'a + Clone + Default> HexaGrid<T> {
    /// Create a new grid of w*h size
    pub fn new(w: usize, h: usize) -> Self {
        let flat_size = h * w + (h / 2);
        HexaGrid::<T> {
            width: w,
            height: h,
            data: vec![<T>::default(); flat_size],
        }
    }

    pub fn iter(&self) -> HexaGridIter<T> {
        HexaGridIter::<T> {
            grid: self,
            index: 0,
        }
    }

    // pub fn iter_mut(&'a mut self) -> HexaGridMutIter<T> {
    //     HexaGridMutIter::<T> { data: self.data.iter_mut() }
    // }

    pub fn get(&self, index: usize) -> &T {
        debug_assert!(index < self.count());
        &self.data[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.count());
        &mut self.data[index]
    }

    pub fn set(&mut self, index: usize, value: T) {
        debug_assert!(index < self.count());
        self.data[index] = value;
    }

    fn is_coordinate_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && y < self.height as i32 &&
        ((y & 0x1 == 0x1 && x < self.width as i32 + 1) || (y & 0x1 == 0x0 && x < self.width as i32))
    }

    fn cardinal_to_location(&self, x: usize, y: usize) -> usize {
        debug_assert!(y<self.height);
        debug_assert!((y & 0x1 == 0x1 && x < self.width + 1) || (y & 0x1 == 0x0 && x < self.width));

        y * self.width + x + (y / 2)
    }

    pub fn get_cardinal(&self, x: usize, y: usize) -> &T {
        &self.data[self.cardinal_to_location(x, y)]
    }

    /// Return the width of the grid
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Return the height of the grid
    pub fn get_height(&self) -> usize {
        self.height
    }
    /// Retrun the numer of element in the grid
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// get the item at the given x,y position. Return None if out of bound
    pub fn get_some(&self, x: i32, y: i32) -> Option<&T> {
        if self.is_coordinate_valid(x, y) {
            Some(self.get_cardinal(x as usize, y as usize))
        } else {
            None
        }
    }
}

impl<T: fmt::Display> fmt::Display for HexaGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Approxiamte capacity : (4 * (w+1)*h)
        // TODO : Currently assume 2 char for T
        let mut res: String = String::with_capacity("[  ]".len() * (self.width + 1) * self.height);
        let mut index = 0;
        for i in 0..self.height {
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


#[cfg(test)]
mod tests {
    use hexagrid::HexaGrid;

    #[test]
    fn test_grid() {
        let mut grid = HexaGrid::<u32>::new(5, 7);
        for i in 0..grid.count() {
            grid.set(i, i as u32);
        }
        assert_eq!(*grid.get(10), 10);
        for i in 0..grid.count() {
            assert_eq!(*grid.get(i), i as u32);
        }
        assert_eq!(grid.get_height(), grid.height);
        assert_eq!(grid.get_width(), grid.width);
    }
    #[test]
    fn grid_count() {
        let grid1 = HexaGrid::<u32>::new(5, 7);
        let grid2 = HexaGrid::<u32>::new(1, 1);
        let grid3 = HexaGrid::<u32>::new(8, 8);
        assert_eq!(grid1.count(), 38);
        assert_eq!(grid2.count(), 1);
        assert_eq!(grid3.count(), 68); // 8*8 + 4
    }
    #[test]
    fn cardinalconversion() {
        let grid = HexaGrid::<u32>::new(5, 7);
        assert_eq!(grid.cardinal_to_location(0, 0), 0);
        assert_eq!(grid.cardinal_to_location(1, 0), 1);
        assert_eq!(grid.cardinal_to_location(1, 1), 6);
        assert_eq!(grid.cardinal_to_location(4, 6), 37);
    }
    #[test]
    fn boundingcheck() {
        let grid = HexaGrid::<u32>::new(5, 7);
        assert_eq!(grid.get_some(-1, 0), None);
        assert_eq!(grid.get_some(0, -1), None);
        assert_eq!(grid.get_some(5, 0), None);
        assert_eq!(grid.get_some(6, 1), None);
        assert_eq!(grid.get_some(5, 7), None);

        assert_eq!(grid.get_some(0, 0), Some(0).as_ref());
        assert_eq!(grid.get_some(4, 0), Some(0).as_ref());
        assert_eq!(grid.get_some(5, 1), Some(0).as_ref());
        assert_eq!(grid.get_some(4, 6), Some(0).as_ref());
    }
}