use std::ops;
use std::fmt;

pub trait Coordinate:
    ops::Add +
    ops::AddAssign +
    ops::Mul +
    ops::Div +
    ops::Rem +
    Clone +
    Copy +
    Eq {}

impl Coordinate for isize {}
impl Coordinate for usize {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord<T: Coordinate>(pub T, pub T);

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

impl<T: Coordinate> ops::Add<Coord<T>> for Coord<T> where T: ops::Add<Output = T> {
    type Output = Coord<T>;

    fn add(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

impl<T: Coordinate> ops::Add<T> for Coord<T> where T: ops::Add<Output = T> {
    type Output = Coord<T>;

    fn add(self, rhs: T) -> Coord<T> {
        Coord(
            self.0 + rhs,
            self.1 + rhs,
        )
    }
}

impl<T: Coordinate> ops::AddAssign<Coord<T>> for Coord<T> {
    fn add_assign(&mut self, rhs: Coord<T>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Coordinate> ops::AddAssign<T> for Coord<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
        self.1 += rhs;
    }
}

impl<T: Coordinate> ops::Sub<Coord<T>> for Coord<T> where T: ops::Sub<Output = T> {
    type Output = Coord<T>;

    fn sub(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 - rhs.0,
            self.1 - rhs.1,
        )
    }
}

impl<T: Coordinate> ops::Sub<T> for Coord<T> where T: ops::Sub<Output = T> {
    type Output = Coord<T>;

    fn sub(self, rhs: T) -> Coord<T> {
        Coord(
            self.0 - rhs,
            self.1 - rhs,
        )
    }
}

impl<T: Coordinate> ops::Mul<Coord<T>> for Coord<T>
    where T: ops::Mul<Output = T> {
    type Output = Coord<T>;

    fn mul(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 * rhs.0,
            self.1 * rhs.1
        )
    }
}

impl<T: Coordinate> ops::Mul<T> for Coord<T>
    where T: ops::Mul<Output = T> {
    type Output = Coord<T>;

    fn mul(self, rhs: T) -> Coord<T> {
        Coord(
            self.0 * rhs,
            self.1 * rhs,
        )
    }
}

impl<T: Coordinate> ops::Div<Coord<T>> for Coord<T>
    where T: ops::Div<Output = T> {
    type Output = Coord<T>;

    fn div(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 / rhs.0,
            self.1 / rhs.1,
        )
    }
}


impl<T: Coordinate> ops::Div<T> for Coord<T>
    where T: ops::Div<Output = T> {
    type Output = Coord<T>;

    fn div(self, rhs: T) -> Coord<T> {
        Coord(
            self.0 / rhs,
            self.1 / rhs,
        )
    }
}

impl<T: Coordinate> ops::Rem<Coord<T>> for Coord<T>
    where T: ops::Rem<Output = T> {
    type Output = Coord<T>;

    fn rem(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 % rhs.0,
            self.1 % rhs.1
        )
    }
}


impl<T: Coordinate> ops::Rem<T> for Coord<T>
    where T: ops::Rem<Output = T> {
    type Output = Coord<T>;

    fn rem(self, rhs: T) -> Coord<T> {
        Coord(
            self.0 % rhs,
            self.1 % rhs,
        )
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

impl Coord<isize> {
    pub fn abs(self) -> Coord<isize> {
        Coord(
            self.0.abs(),
            self.1.abs(),
        )
    }
}