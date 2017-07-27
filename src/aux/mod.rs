pub mod index_iter;
pub mod coord;

pub trait ModuloSignedExt<RHS = Self> {
    type Output;
    fn modulo(self, n: RHS) -> Self::Output;
}

macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt<Self> for $t {
            type Output = Self;
            
            #[inline]
            fn modulo(self, rhs: Self) -> Self::Output {
                ((self % rhs) + rhs)%rhs
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 isize }


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
                let negative = (self<0) as Self;
                ((self + negative) / rhs) - negative
            }
        }
    )*)
}
div_floor_signed_ext_impl! { i8 i16 i32 i64 isize }
