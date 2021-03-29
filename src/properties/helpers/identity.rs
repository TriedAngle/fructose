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

impl<T> One for T where T: Identity<Multiplicative> + Sized + PartialEq { }

impl<T> Zero for T where T: Identity<Additive> + Sized + PartialEq { }

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