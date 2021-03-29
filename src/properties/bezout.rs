use crate::properties::gcd::GCD;

pub trait Bezout: GCD {
    #[inline]
    fn bezout_coefficients(self, rhs: Self) -> (Self, Self) {
        let (a, b, _) = self.bezout(rhs);
        (a, b)
    }

    fn bezout(&self, rhs: Self) -> (Self, Self, Self);
}

macro_rules! impl_bezout {
    ($($set:ty)*) => {
        $(
            impl Bezout for $set {
                #[inline]
                fn bezout(&self, _rhs: Self) -> (Self, Self, Self) {
                    unimplemented!()
                }
            }
        )*
    }
}

impl_bezout!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
