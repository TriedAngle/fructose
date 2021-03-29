use crate::algebra::ring::{IntegerRing, NaturalCommutativeSemiring};
use crate::cast::{FromI32, FromU32};
use crate::operators::mul_add::{MulAdd, MulAddAssign};
use crate::properties::helpers::bound::Bounded;
use crate::properties::helpers::identity::{One, Zero};

// NOTE: right now everything can be used as a natural number, so this needs to be fixed asap.
pub trait Natural:
    NaturalCommutativeSemiring + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32
{
}

pub trait Integer:
    IntegerRing + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32 + FromI32
{
}

impl<T> Natural for T where
    T: NaturalCommutativeSemiring + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32
{
}

impl<T> Integer for T where
    T: IntegerRing + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32 + FromI32
{
}
