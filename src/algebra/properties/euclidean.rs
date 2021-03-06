use crate::algorithms::euclidean::extended_euclidean;
use crate::specific::natural::Natural;

pub trait EuclideanDiv: Sized {
    type Norm: Natural;

    fn euclid_norm(&self) -> Self::Norm;

    fn div_euclid_quotient(&self, rhs: Self) -> Self {
        let (quot, _) = self.div_euclid(rhs);
        quot
    }

    fn div_euclid_remainder(&self, rhs: Self) -> Self {
        let (_, rem) = self.div_euclid(rhs);
        rem
    }

    fn div_euclid(&self, rhs: Self) -> (Self, Self);
}

// TODO: use Natural Numbers instead of Self
macro_rules! impl_euclid_div {
    ($($set:ty)*) => {
        $(
            impl EuclideanDiv for $set {
                type Norm = $set;

                #[inline]
                fn euclid_norm(&self) -> Self::Norm {
                    *self
                }

                #[inline]
                fn div_euclid(&self, rhs: Self) -> (Self, Self) {
                    let (a, b, _) = extended_euclidean(*self, rhs);
                    (a, b)
                }
            }
        )*
    }
}

impl_euclid_div!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
