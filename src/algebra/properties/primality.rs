use crate::algorithms::factorization::is_prime as is_prime_algo;

// TODO: make this better
pub trait Primality {
    fn is_irreducible(&self) -> bool;
    fn is_prime(&self) -> bool;
}

macro_rules! impl_primality {
    ($($set:ty)*) => {
        $(
            impl Primality for $set {
                #[inline]
                fn is_irreducible(&self) -> bool {
                    unimplemented!()
                }

                #[inline]
                fn is_prime(&self) -> bool {
                    is_prime_algo(*self as u64)
                }
            }
        )*
    }
}

impl_primality!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
