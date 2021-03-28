use crate::algebra::helpers::bound::Bounded;
use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{EuclideanSemidomain, IntegerRing, NaturalCommutativeSemiring};
use crate::cast::{FromI32, FromU32};
use crate::operators::bit::ClosedBitOps;
use crate::operators::mul_add::{MulAdd, MulAddAssign};
use crate::operators::{Additive, ClosedOps, Multiplicative};

// NOTE: right now everything can be used as a natural number, so this needs to be fixed asap.
pub trait Natural:
    NaturalCommutativeSemiring + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32
{
}

pub trait Integer:
    IntegerRing + Bounded + MulAdd + MulAddAssign + Zero + One + FromU32 + FromI32
{
}

pub trait Bits:
    ClosedBitOps
    + ClosedOps
    + EuclideanSemidomain
    + Primality
    + ArchimedeanDiv
    + Bounded
    + Zero
    + One
    + Ordered<Additive>
    + Ordered<Multiplicative>
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

impl<T> Bits for T where
    T: ClosedBitOps
        + ClosedOps
        + EuclideanSemidomain
        + Primality
        + ArchimedeanDiv
        + Bounded
        + Zero
        + One
        + Ordered<Additive>
        + Ordered<Multiplicative>
{
}
