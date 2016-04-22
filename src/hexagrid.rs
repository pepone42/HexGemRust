use std::fmt;
use rand::{Rand, random};
use std::slice;

#[derive(Debug)]
pub struct HexaGrid<T> {
    width: usize,
    heigth: usize,
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

pub struct HexaGridMutIterUnsafe<'a, T: 'a> {
    data: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for HexaGridMutIterUnsafe<'a, T> {
    type Item = &'a mut T;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            None
        } else {
            self.index += 1;
            // TODO : See how to do it without unsafe.
            //
            // Original Code :
            // Some(&mut self.data[self.index - 1])
            // -> Error : error: cannot infer an appropriate lifetime for lifetime
            //            parameter in function call due to conflicting requirements [E0495]
            // Explanation here:
            // https://m.reddit.com/r/rust/comments/4f9dxu/can_you_use_iterators_for_a_sudoku_solver/
            // IIUC, we return an Option<Self::Item> with a lifetime 'a, the input self has a lifetime 'b
            // Rust can't know if 'a outlive 'b
            //
            // unsafe method :
            // by creating a raw pointer to our Cell, then dereference and return it

            let a = &mut self.data[self.index - 1] as *mut _;
            Some(unsafe { &mut *a })
        }
    }
}

pub struct HexaGridMutIter<'a, T: 'a> {
    data: slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for HexaGridMutIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|item| item)
    }
}

impl<'a, T: 'a + Clone + Default> HexaGrid<T> {
    pub fn new(w: usize, h: usize) -> Self {
        let flat_size = h * w + (h / 2);
        HexaGrid::<T> {
            width: w,
            heigth: h,
            data: vec![<T>::default(); flat_size],
        }
    }
    pub fn iter(&self) -> HexaGridIter<T> {
        HexaGridIter::<T> {
            grid: self,
            index: 0,
        }
    }
    pub fn iter_mut_unsafe(&'a mut self) -> HexaGridMutIterUnsafe<T> {
        HexaGridMutIterUnsafe::<T> {
            data: &mut self.data,
            index: 0,
        }
    }
    pub fn iter_mut(&'a mut self) -> HexaGridMutIter<T> {
        HexaGridMutIter::<T> { data: self.data.iter_mut() }
    }
    // TODO : Move into a parent struct.
    // pub fn randomize(&mut self) {
    //     for i in 0..self.data.len() {
    //         self.data[i] = random::<T>();
    //     }
    // }
}

impl<T: fmt::Display> fmt::Display for HexaGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Approxiamte capacity : (4 * (w+1)*h)
        // TODO : Currently assume 2 char for T
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
