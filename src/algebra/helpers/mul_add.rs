use crate::algebra::properties::general::Set;
use crate::operators::{Additive, Multiplicative};

pub trait MulAdd<A = Self, B = Self>: Set<Additive> + Set<Multiplicative> {
    type Output;

    fn mul_add(self, a: A, b: B) -> Self::Output;
}

pub trait MulAddAssign<A = Self, B = Self>: Set<Additive> + Set<Multiplicative> {
    fn mul_add_assign(&mut self, a: A, b: B);
}

macro_rules! impl_mul_add {
    ($($set:ty)*) => {
        $(
            impl MulAdd for $set {
                type Output = $set;

                fn mul_add(self, a: $set, b: $set) -> Self::Output {
                    self.mul_add(a, b)
                }
            }

            impl MulAddAssign for $set {
                fn mul_add_assign(&mut self, a: $set, b: $set) {
                    *self = self.mul_add(a, b);
                }
            }
        )*
    };
    ($($set:ty)* => int) => {
        $(
            impl MulAdd for $set {
                type Output = $set;

                fn mul_add(self, a: $set, b: $set) -> Self::Output {
                    (self * a) + b
                }
            }

            impl MulAddAssign for $set {
                fn mul_add_assign(&mut self, a: $set, b: $set) {
                    *self = (*self * a) + b
                }
            }
        )*
    }
}

impl_mul_add!(f32 f64);
impl_mul_add!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 => int);
