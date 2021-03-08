use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{IntegerRing, NaturalCommutativeSemiring, SemiEuclideanDomain};
use crate::operators::bit::ClosedBitOps;
use crate::operators::{
    Additive, ClosedAdd, ClosedMul, ClosedNeg, ClosedOps, ClosedRem, ClosedSub, Multiplicative,
};
use crate::algebra::helpers::bound::Bounded;

// NOTE: right now everything can be used as a natural number, so this needs to be fixed asap.
pub trait Natural:
    NaturalCommutativeSemiring + Bounded + Zero + One
{
}

pub trait Integer:
    IntegerRing  + Bounded + Zero + One
{
}

pub trait Bits:
    ClosedBitOps
    + ClosedOps
    + SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Bounded
    + Zero
    + One
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
}

impl<T> Natural for T where T: NaturalCommutativeSemiring + Bounded + Zero + One {}

impl<T> Integer for T where T: IntegerRing + Bounded + Zero + One {}

impl<T> Bits for T where
    T: ClosedBitOps
        + ClosedOps
        + SemiEuclideanDomain
        + Primality
        + ArchimedeanDiv
        + Bounded
        + Zero
        + One
        + Ordered<Additive>
        + Ordered<Multiplicative>
{
}
