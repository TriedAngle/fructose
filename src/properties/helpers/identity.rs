use crate::operators::{Additive, Multiplicative};
use crate::properties::general::Identity;

pub trait One: Identity<Multiplicative> + Sized + PartialEq {
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

pub trait Zero: Identity<Additive> + Sized + PartialEq {
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

impl<T> One for T where T: Identity<Multiplicative> + Sized + PartialEq {}

impl<T> Zero for T where T: Identity<Additive> + Sized + PartialEq {}

pub trait Two: Sized + PartialEq {
    fn two() -> Self;

    fn set_two(&mut self) {
        *self = Self::two();
    }

    fn is_two(&self) -> bool {
        *self == Self::two()
    }
}

pub trait AnyInt: Sized {
    fn any_num<const N: i32>() -> Self;

    fn set_any_num<const N: i32>(&mut self);

    fn is_any_num<const N: i32>(&self) -> bool;
}

macro_rules! impl_two {
    ($($set:ty => $val:expr)*) => {
        $(
            impl Two for $set {
                fn two() -> Self {
                    $val
                }
            }
        )*
    }
}

impl_two! {
    u8      => 2
    u16     => 2
    u32     => 2
    u64     => 2
    u128    => 2
    usize   => 2
    i8      => 2
    i16     => 2
    i32     => 2
    i64     => 2
    i128    => 2
    isize   => 2
    f32     => 2.0
    f64     => 2.0
}
