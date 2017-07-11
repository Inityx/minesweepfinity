use std::ops;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Coord<T: Coordinate>(pub T, pub T);

impl<T: Coordinate> ops::Add for Coord<T> where T: ops::Add<Output = T> {
    type Output = Coord<T>;

    fn add(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl<'a, T: Coordinate> ops::Add<Coord<T>> for &'a Coord<T> where T: ops::Add<Output = T> {
    type Output = Coord<T>;

    fn add(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl<T: Coordinate> ops::AddAssign<Coord<T>> for Coord<T> {
    fn add_assign(&mut self, rhs: Coord<T>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Coordinate> ops::Sub for Coord<T> where T: ops::Sub<Output = T> {
    type Output = Coord<T>;

    fn sub(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 - rhs.0,
            self.1 - rhs.1
        )
    }
}

impl<T: Coordinate> ops::Mul for Coord<T>
    where T: ops::Mul<Output = T> {
    type Output = Coord<T>;

    fn mul(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 * rhs.0,
            self.1 * rhs.1
        )
    }
}

impl<T: Coordinate> ops::Div for Coord<T>
    where T: ops::Div<Output = T> {
    type Output = Coord<T>;

    fn div(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 / rhs.0,
            self.1 / rhs.1
        )
    }
}

impl<T: Coordinate> ops::Rem for Coord<T>
    where T: ops::Rem<Output = T> {
    type Output = Coord<T>;

    fn rem(self, rhs: Coord<T>) -> Coord<T> {
        Coord(
            self.0 % rhs.0,
            self.1 % rhs.1
        )
    }
}

impl From<Coord<usize>> for Coord<isize> {
    fn from(rhs: Coord<usize>) -> Self {
        Coord(
            rhs.0 as isize,
            rhs.1 as isize,
        )
    }
}

impl From<Coord<isize>> for Coord<usize> {
    fn from(rhs: Coord<isize>) -> Self {
        Coord(
            rhs.0 as usize,
            rhs.1 as usize,
        )
    }
}
