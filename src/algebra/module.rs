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
