use std::ops::*;
use std::fmt;

use super::ModuloSignedExt;
use super::DivFloorSignedExt;

pub trait Coordinate:
    Add +
    AddAssign +
    Mul +
    Div +
    Rem +
    Clone +
    Copy +
    Eq {}

impl Coordinate for isize {}
impl Coordinate for usize {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord<T: Coordinate>(pub T, pub T);


// FMT
impl<T: fmt::Display + Coordinate> fmt::Display for Coord<T> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: fmt::Debug + Coordinate> fmt::Debug for Coord<T> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}


// ops
macro_rules! coord_op_self_impl {
    ( $(($trait: ident, $func: ident),)* ) => ( $(
        impl<T: Coordinate> $trait<Self> for Coord<T> where T: $trait<Output = T> {
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

coord_op_self_impl! {
    (Add, add),
    (Sub, sub),
    (Mul, mul),
    (Div, div),
    (Rem, rem),
    (ModuloSignedExt, modulo),
    (DivFloorSignedExt, div_floor),
}

macro_rules! coord_op_t_impl {
    ( $(($trait: ident, $func: ident),)* ) => ( $(
        impl<T: Coordinate> $trait<T> for Coord<T> where T: $trait<Output = T> {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: T) -> Self {
                Coord(
                    $trait::$func(self.0, rhs),
                    $trait::$func(self.1, rhs),
                )
            }
        }
    )* )
}

coord_op_t_impl! {
    (Add, add),
    (Sub, sub),
    (Mul, mul),
    (Div, div),
    (Rem, rem),
    (ModuloSignedExt, modulo),
    (DivFloorSignedExt, div_floor),
}

impl<T: Coordinate> AddAssign<Self> for Coord<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Coordinate> AddAssign<T> for Coord<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
        self.1 += rhs;
    }
}


// From<Self<T>>
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


// impl
impl<T: Coordinate> Coord<T> where T: Add<Output = T> {
    pub fn sum(self) -> T {
        self.0 + self.1
    }
}

impl Coord<isize> {
    pub fn abs(self) -> Self {
        Coord(
            self.0.abs(),
            self.1.abs(),
        )
    }
}
