#![allow(dead_code)]
use super::coord::Coord;
use ::std::slice;
// TODO make macro

pub const CARDINAL_OFFSETS: &'static [Coord<isize>;4] = &[
    Coord(-1, 0),
    Coord( 1, 0),
    Coord( 0,-1),
    Coord( 0, 1)
];

pub struct IndexIterSigned {
    count: isize,
    limit: isize,
    dim_one: isize,
    offset: Coord<isize>,
}

impl IndexIterSigned {
    pub fn new(dimension: Coord<isize>, offset: Coord<isize>) -> IndexIterSigned {
        IndexIterSigned {
            count: 0,
            limit: (dimension.0 * dimension.1),
            dim_one: dimension.1,
            offset: offset,
        }
    }
}

impl Iterator for IndexIterSigned {
    type Item = Coord<isize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit { return None; }
        
        let ret = Coord(
            self.count/self.dim_one + self.offset.0,
            self.count%self.dim_one + self.offset.1,
        );
        
        self.count += 1;
        Some(ret)
    }
}


pub struct IndexIterUnsigned {
    count: usize,
    limit: usize,
    dim_one: usize,
    offset: Coord<usize>
}

impl IndexIterUnsigned {
    pub fn new(dimension: Coord<usize>, offset: Coord<usize>) -> IndexIterUnsigned {
        IndexIterUnsigned {
            count: 0,
            limit: (dimension.0 * dimension.1),
            dim_one: dimension.1,
            offset: offset,
        }
    }
}

impl Iterator for IndexIterUnsigned {
    type Item = Coord<usize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit { return None; }
        
        let ret = Coord(
            self.count/self.dim_one + self.offset.0,
            self.count%self.dim_one + self.offset.1,
        );
        
        self.count += 1;
        Some(ret)
    }
}

pub fn self_and_adjacent() -> IndexIterSigned {
    IndexIterSigned::new(Coord(3,3), Coord(-1,-1))
}

pub fn cardinal_adjacent() -> slice::Iter<'static, Coord<isize>> {
    CARDINAL_OFFSETS.iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    
    #[test]
    fn eight_square_index_unsigned() {
        let index_vec = IndexIterUnsigned::new(Coord(8,8), Coord(0,0)).collect::<Vec<Coord<usize>>>();
        let mut full_vec = Vec::with_capacity(64);
        
        for i in 0..8 {
            for j in 0..8 {
                full_vec.push(Coord(i, j));
            }
        }
        
        assert_eq!(index_vec, full_vec);
    }
    
    #[test]
    fn eight_square_index_signed() {
        let index_vec = IndexIterSigned::new(Coord(8,8), Coord(-4,-4)).collect::<Vec<Coord<isize>>>();
        let mut full_vec = Vec::with_capacity(64);
        
        for i in -4..4 {
            for j in -4..4 {
                full_vec.push(Coord(i, j));
            }
        }
        
        assert_eq!(index_vec, full_vec);
    }
    
    #[test]
    fn adjacent_index() {
        let index_vec = super::self_and_adjacent().collect::<Vec<Coord<isize>>>();
        let mut full_vec = Vec::with_capacity(9);
        
        for i in -1..2 {
            for j in -1..2 {
                full_vec.push(Coord(i, j));
            }
        }
        
        assert_eq!(index_vec, full_vec);
    }
    
    #[test]
    fn nonsquare_unsigned() {
        let index_vec = IndexIterUnsigned::new(Coord(3,5), Coord(0,0)).collect::<Vec<Coord<usize>>>();
        let mut full_vec = Vec::with_capacity(15);
        
        for i in 0..3 {
            for j in 0..5 {
                full_vec.push(Coord(i,j));
            }
        }
        
        assert_eq!(index_vec, full_vec);
    }
    
    #[test]
    fn nonsquare_signed() {
        let index_vec = IndexIterSigned::new(Coord(3,5), Coord(-1,-2)).collect::<Vec<Coord<isize>>>();
        let mut full_vec = Vec::with_capacity(15);
        
        for i in -1..2 {
            for j in -2..3 {
                full_vec.push(Coord(i,j));
            }
        }
        
        assert_eq!(index_vec, full_vec);
    }
}
