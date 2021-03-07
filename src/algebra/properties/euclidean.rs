use crate::specific::int::Natural;

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

macro_rules! impl_euclidean_int {
    // Helper
    (@UNIT $val:expr => @natural) => {
        $val as Self::Norm
    };
    (@UNIT $val:expr => @integer) => {
        $val.abs() as Self::Norm
    };
    // Entrypoint
    ($($signed:ident:$unsigned:ident), *) => {
        $(
            impl_euclidean_int!(@NEXT $unsigned : $signed : $unsigned @natural);
            impl_euclidean_int!(@NEXT $signed : $signed : $unsigned @integer);
        )*
    };
    (@NEXT $set:ident : $signed:ident : $unsigned:ident $($tt:tt)*) => {
        impl EuclideanDiv for $set {
            type Norm = $unsigned;

            #[inline]
            fn euclid_norm(&self) -> Self::Norm {
                impl_euclidean_int!(@UNIT *self => $($tt)*)
            }

            #[inline]
            fn div_euclid(&self, rhs: Self) -> (Self, Self) {
                (self / rhs, self % rhs)
            }
        }
    };
}

impl_euclidean_int! {
    i8    : u8,
    i16   : u16,
    i32   : u32,
    i64   : u64,
    i128  : u128,
    isize : usize
}
