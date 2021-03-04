use crate::group::AbelianGroup;
use crate::operators::{Additive, Multiplicative, Operator};
use crate::ring::{CommutativeRing, Ring};

pub trait Module<A: Operator = Additive, RA: Operator = Additive, RM: Operator = Multiplicative>:
    AbelianGroup<A>
{
    type Ring: Ring<RA, RM>;
}

pub trait CommutativeModule: Module<Ring = <Self as CommutativeModule>::Ring> {
    type Ring: CommutativeRing;
}
