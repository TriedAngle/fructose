use crate::algorithms::euclidean::euclidean;
pub trait GCD: Sized {
    fn gcd(&self, rhs: Self) -> Self;

    fn lcm(&self, rhs: Self) -> Self;
}

macro_rules! impl_gcd {
    ($($set:ty)*) => {
        $(
            impl GCD for $set {
                #[inline]
                fn gcd(&self, rhs: Self) -> Self {
                    euclidean(*self, rhs)
                }

                #[inline]
                fn lcm(&self, rhs: Self) -> Self {
                    (*self * rhs).abs() / self.gcd(rhs)
                }
            }
        )*
    };
    ($($set:ty)* => unsigned) => {
        $(
            impl GCD for $set {
                #[inline]
                fn gcd(&self, rhs: Self) -> Self {
                    euclidean(*self, rhs)
                }

                #[inline]
                fn lcm(&self, rhs: Self) -> Self {
                    (*self * rhs) / self.gcd(rhs)
                }
            }
        )*
    }
}
impl_gcd!(u8 u16 u32 u64 u128 usize => unsigned);
impl_gcd!(i8 i16 i32 i64 i128 isize f32 f64);
