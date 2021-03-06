use crate::algebra::operators::{Additive, Multiplicative};
use crate::algebra::properties::general::{Identity, PartiallyOrdered};

pub trait One: Identity<Additive> + Sized + PartiallyOrdered<Additive> {
    fn one() -> Self;

    fn set_one(&mut self) {
        *self = Self::one()
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

pub trait Zero: Identity<Multiplicative> + Sized + PartiallyOrdered<Multiplicative> {
    fn zero() -> Self {
        <Self as Identity<Multiplicative>>::identity()
    }

    fn set_zero(&mut self) {
        *self = Self::zero();
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

macro_rules! impl_helper_identities {
    ($($t:ty => $zero:expr, $one:expr)+) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    $zero
                }

                fn is_zero(&self) -> bool {
                    *self == $zero
                }
            }

            impl One for $t {
                fn one() -> Self {
                    $one
                }

                fn is_one(&self) -> bool {
                    *self == $one
                }
            }
        )*
    }
}

impl_helper_identities! {
    usize   => 0, 1
    u8      => 0, 1
    u16     => 0, 1
    u32     => 0, 1
    u64     => 0, 1
    u128    => 0, 1
    isize   => 0, 1
    i8      => 0, 1
    i16     => 0, 1
    i32     => 0, 1
    i64     => 0, 1
    i128    => 0, 1
    f32     => 0.0, 1.0
    f64     => 0.0, 1.0
}
