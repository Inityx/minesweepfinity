use super::coord::Coord;

// TODO make macro

pub struct IndexIterSigned {
    count: isize,
    limit: isize,
    dim_zero: isize,
    offset: Coord<isize>,
}

impl IndexIterSigned {
    pub fn new(dimension: Coord<isize>, offset: Coord<isize>) -> IndexIterSigned {
        IndexIterSigned {
            count: 0,
            limit: (dimension.0 * dimension.1),
            dim_zero: dimension.0,
            offset: offset,
        }
    }
}

impl Iterator for IndexIterSigned {
    type Item = Coord<isize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit { return None; }
        
        let ret = Coord(
            self.count/self.dim_zero + self.offset.0,
            self.count%self.dim_zero + self.offset.1,
        );
        
        self.count += 1;
        Some(ret)
    }
}


pub struct IndexIterUnsigned {
    count: usize,
    limit: usize,
    dim_zero: usize,
    offset: Coord<usize>
}

impl IndexIterUnsigned {
    pub fn new(dimension: Coord<usize>, offset: Coord<usize>) -> IndexIterUnsigned {
        IndexIterUnsigned {
            count: 0,
            limit: (dimension.0 * dimension.1),
            dim_zero: dimension.0,
            offset: offset,
        }
    }
}

impl Iterator for IndexIterUnsigned {
    type Item = Coord<usize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit { return None; }
        
        let ret = Coord(
            self.count/self.dim_zero + self.offset.0,
            self.count%self.dim_zero + self.offset.1,
        );
        
        self.count += 1;
        Some(ret)
    }
}

pub fn adjacent() -> IndexIterSigned {
    IndexIterSigned::new(Coord(3,3), Coord(-1,-1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    
    #[test]
    fn chunk_index() {
        let index_iter = IndexIterUnsigned::new(Coord(8,8), Coord(0,0));
        let mut full_vec = Vec::with_capacity(64);
        
        for i in 0..8 {
            for j in 0..8 {
                full_vec.push(Coord(i, j));
            }
        }
        
        let index_vec = index_iter.collect::<Vec<Coord<usize>>>();
        assert_eq!(index_vec, full_vec);
    }
    
    #[test]
    fn adjacent_index() {
        let index_iter = super::adjacent();
        let mut full_vec = Vec::with_capacity(9);
        
        for i in -1..2 {
            for j in -1..2 {
                full_vec.push(Coord(i, j));
            }
        }
        
        let index_vec = index_iter.collect::<Vec<Coord<isize>>>();
        assert_eq!(index_vec, full_vec);
    }
}