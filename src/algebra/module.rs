use crate::algebra::group::AbelianGroup;
use crate::algebra::operators::{Additive, Multiplicative, Operator};
use crate::algebra::ring::{CommutativeRing, Ring};

pub trait Module<A: Operator = Additive, RA: Operator = Additive, RM: Operator = Multiplicative>:
    AbelianGroup<A>
{
    type Ring: Ring<RA, RM>;
}

pub trait CommutativeModule: Module<Ring = <Self as CommutativeModule>::Ring> {
    type Ring: CommutativeRing;
}

impl_module!(i8 i16 i32 i64 isize f32 f64);
impl_commutative_module!(i8 i16 i32 i64 isize f32 f64);
