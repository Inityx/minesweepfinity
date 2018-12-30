pub mod index_iter;
pub mod coord;

use std::ops::{Add, Sub, Rem, Div};

pub trait ModuloSignedExt<Rhs=Self> {
    type Output;
    fn modulo(self, rhs: Rhs) -> Self::Output;
}

impl<T, U> ModuloSignedExt<U> for T
where
    U: Clone,
    Self: Rem<U>,
    <Self as Rem<U>>::Output: Add<U>,
    <<Self as Rem<U>>::Output as Add<U>>::Output: Rem<U>,
{
    type Output = <<<Self as Rem<U>>::Output as Add<U>>::Output as Rem<U>>::Output;

    fn modulo(self, rhs: U) -> Self::Output {
        self
            .rem(rhs.clone())
            .add(rhs.clone())
            .rem(rhs)
    }
}

pub trait DivFloorSignedExt<RHS = Self> {
    type Output;
    fn div_floor(self, n: RHS) -> Self::Output;
}

macro_rules! div_floor_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl DivFloorSignedExt<Self> for $t {
            type Output = Self;

            #[inline]
            fn div_floor(self, rhs: Self) -> Self::Output {
                if self.is_negative() {
                    self.add(1).div(rhs).sub(1)
                } else {
                    self.div(rhs)
                }
            }
        }
    )*)
}

div_floor_signed_ext_impl! { isize }

pub trait OptionalizeExt: Sized {
    fn optionalize(self) -> Option<Self>;
}

impl<T> OptionalizeExt for Vec<T> {
    fn optionalize(self) -> Option<Self> {
        if self.is_empty() { None } else { Some(self) }
    }
}