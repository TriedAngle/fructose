use crate::algebra::group::AbelianGroup;
use crate::algebra::ring::{CommutativeRing, Ring};
use crate::operators::{Additive, Multiplicative, Operator};

pub trait Module<A: Operator = Additive, RA: Operator = Additive, RM: Operator = Multiplicative>:
    AbelianGroup<A>
{
    type Ring: Ring<RA, RM>;
}

pub trait CommutativeModule: Module<Ring = <Self as CommutativeModule>::Ring> {
    type Ring: CommutativeRing;
}

macro_rules! impl_module {
    ($($set:ident)*) => {
        $(
            impl Module for $set {
                type Ring = $set;
            }
        )*
    }
}

macro_rules! impl_commutative_module {
    ($($set:ident)*) => {
        $(
            impl CommutativeModule for $set {
                type Ring = $set;
            }
        )*
    }
}

impl_module!(i8 i16 i32 i64 isize f32 f64);
impl_commutative_module!(i8 i16 i32 i64 isize f32 f64);
