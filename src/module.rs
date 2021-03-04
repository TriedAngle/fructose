use crate::operators::{Additive, Operator, Multiplicative};
use crate::group::AbelianGroup;
use crate::ring::{Ring, CommutativeRing};

pub trait Module<
    A: Operator = Additive,
    RA: Operator = Additive,
    RM: Operator = Multiplicative
>: AbelianGroup<A> + Ring<RA, RM>
{
}

pub trait CommutativeModule<
    A: Operator = Additive,
    RA: Operator = Additive,
    RM: Operator = Multiplicative
>: AbelianGroup<A> + CommutativeRing<RA, RM>
{
}