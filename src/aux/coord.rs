use std::ops::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord<T>(pub T, pub T);

impl<T> Coord<T> {
    pub fn sum(self) -> T where T: Add<Output=T> {
        self.0 + self.1
    }

    pub fn map<U>(self, mut func: impl FnMut(T) -> U) -> Coord<U> {
        let Coord(x, y) = self;
        Coord(func(x), func(y))
    }
}

impl<T: fmt::Debug> fmt::Debug for Coord<T> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}

impl<T: fmt::Display> fmt::Display for Coord<T> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

macro_rules! coord_op_impl {
    ( $(($trait: ident, $func: ident),)* ) => ( $(
        // Self
        impl<T> $trait<Self> for Coord<T> where T: $trait<Output = T> {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Coord(
                    $trait::$func(self.0, rhs.0),
                    $trait::$func(self.1, rhs.1),
                )
            }
        }
    )* )
}

coord_op_impl! {
    (Add, add),
    (Sub, sub),
    (Mul, mul),
    (Div, div),
    (Rem, rem),
}

impl<T: AddAssign> AddAssign<Self> for Coord<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Coord<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.clone();
        self.1 += rhs;
    }
}


impl From<Coord<usize>> for Coord<isize> {
    fn from(src: Coord<usize>) -> Self {
        Coord(
            src.0 as isize,
            src.1 as isize,
        )
    }
}

impl From<Coord<isize>> for Coord<usize> {
    fn from(src: Coord<isize>) -> Self {
        Coord(
            src.0 as usize,
            src.1 as usize,
        )
    }
}

impl<T> From<(T, T)> for Coord<T> {
    fn from((x, y): (T, T)) -> Self { Coord(x, y) }
}

impl Coord<isize> {
    pub fn abs(self) -> Self { self.map(isize::abs) }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sanity() {
        let coord = Coord(5,6);
        
        assert_eq!(5, coord.0);
        assert_eq!(6, coord.1);
    }
}
