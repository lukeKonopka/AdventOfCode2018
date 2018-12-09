use smallbitvec::SmallBitVec;
use super::rect::Rect;

pub struct Canvas {
    n: usize,
    base: SmallBitVec,
    overlap: SmallBitVec,
}

impl Canvas {
    pub fn create(n: usize) -> Self {
        let base = SmallBitVec::from_elem(n*n, false);
        let overlap = SmallBitVec::from_elem(n*n, false);
        Canvas { n, base, overlap }
    }

    fn paint_el(&mut self, x: usize, y: usize) {
        let index = y * self.n + x;
        let base_el = self.base.get(index);
        match base_el {
            Some(true) => self.overlap.set(index, true),
            Some(false) => self.base.set(index, true),
            None => {}
        };
    }

    pub fn paint(&mut self, rect: &Rect) {
        let (x_start, y_start, x_end, y_end) = rect.get_bounds();
        for x in x_start..x_end {
            for y in y_start..y_end {
                self.paint_el(x as usize, y as usize);
            }
        }
    }

    pub fn get_overlap_area(&self) -> usize {
        self.overlap.iter().fold(0, |acc, value| if value { acc + 1 } else { acc })
    }

    pub fn is_overlap(&self, rect: &Rect) -> bool {
        let (x_start, y_start, x_stop, y_stop) = rect.get_bounds();
        for x in x_start..x_stop {
            for y in y_start..y_stop {
                let index = y as usize * self.n + x as usize;
                if self.overlap.get(index).unwrap_or(false) {
                    return true;
                }
            }
        }
        false
    }
}
