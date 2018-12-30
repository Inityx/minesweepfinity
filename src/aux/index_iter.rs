#![allow(dead_code)]
use crate::aux::coord::Coord;
// TODO make macro

const CARDINAL_OFFSETS: &'static [Coord<isize>;4] = &[
    Coord(-1, 0),
    Coord( 1, 0),
    Coord( 0,-1),
    Coord( 0, 1),
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
    
    pub fn self_and_adjacent(origin: impl Into<Coord<isize>>) -> Self {
        IndexIterSigned::new(Coord(3,3), origin.into() + Coord(-1,-1))
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

pub fn cardinal_adjacent() -> impl Iterator {
    CARDINAL_OFFSETS.iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    use itertools::iproduct;
    
    #[test]
    fn index_unsigned() {
        assert_eq!(
            iproduct!(0..8, 0..8).map(Coord::from).collect::<Vec<_>>(),
            IndexIterUnsigned::new(Coord(8,8), Coord(0,0)).collect::<Vec<_>>(),
        );
    }
    
    #[test]
    fn index_signed() {
        assert_eq!(
            iproduct!(-4..4, -4..4).map(Coord::from).collect::<Vec<_>>(),
            IndexIterSigned::new(Coord(8,8), Coord(-4,-4)).collect::<Vec<_>>(),
        );
    }
    
    #[test]
    fn adjacent_index() {
        assert_eq!(
            iproduct!(-1..2, -1..2).map(Coord::from).collect::<Vec<_>>(),
            IndexIterSigned::self_and_adjacent(Coord::<isize>(0,0)).collect::<Vec<_>>(),
        );
    }
    
    #[test]
    fn nonsquare_unsigned() {
        assert_eq!(
            iproduct!(0..3, 0..5).map(Coord::from).collect::<Vec<_>>(),
            IndexIterUnsigned::new(Coord(3,5), Coord(0,0)).collect::<Vec<_>>(),
        );
    }
    
    #[test]
    fn nonsquare_signed() {
        assert_eq!(
            iproduct!(-1..2, -2..3).map(Coord::from).collect::<Vec<_>>(),
            IndexIterSigned::new(Coord(3,5), Coord(-1,-2)).collect::<Vec<_>>(),
        );
    }
}
