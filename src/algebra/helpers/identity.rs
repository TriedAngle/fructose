use crate::algebra::properties::general::{Identity, PartiallyOrdered};
use crate::operators::{Additive, Multiplicative};

pub trait One: Identity<Multiplicative> + Sized + PartiallyOrdered<Multiplicative> {
    fn one() -> Self {
        <Self as Identity<Multiplicative>>::identity()
    }

    fn set_one(&mut self) {
        *self = Self::one()
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

pub trait Zero: Identity<Additive> + Sized + PartiallyOrdered<Additive> {
    fn zero() -> Self {
        <Self as Identity<Additive>>::identity()
    }

    fn set_zero(&mut self) {
        *self = Self::zero();
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

pub trait Two: Sized + PartialEq {
    fn two() -> Self;

    fn set_two(&mut self) {
        *self = Self::two();
    }

    fn is_two(&self) -> bool {
        *self == Self::two()
    }
}

#[cfg(feature = "const")]
pub trait AnyInt: Sized {
    fn any_num<const N: i32>() -> Self;

    fn set_any_num<const N: i32>(&mut self);

    fn is_any_num<const N: i32>(&self) -> bool;
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
