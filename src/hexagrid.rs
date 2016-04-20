use std::fmt;
use rand::{Rand, random};

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
