use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{IntegerRing, NaturalCommutativeSemiring, SemiEuclideanDomain};
use crate::operators::bit::ClosedBitOps;
use crate::operators::{
    Additive, ClosedAdd, ClosedMul, ClosedNeg, ClosedOps, ClosedRem, ClosedSub, Multiplicative,
};

// NOTE: right now everything can be used as a natural number, so this needs to be fixed asap.
pub trait Natural:
    NaturalCommutativeSemiring + ClosedAdd + ClosedMul + ClosedRem + Zero + One
{
}

pub trait Integer:
    IntegerRing + ClosedAdd + ClosedSub + ClosedMul + ClosedRem + ClosedNeg + Zero + One
{
}

pub trait Bits:
    ClosedBitOps
    + ClosedOps
    + SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Zero
    + One
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
}

impl<T> Natural for T where T: NaturalCommutativeSemiring + Zero + One {}

impl<T> Integer for T where T: IntegerRing + Zero + One {}

impl<T> Bits for T where
    T: ClosedBitOps
        + ClosedOps
        + SemiEuclideanDomain
        + Primality
        + ArchimedeanDiv
        + Zero
        + One
        + Ordered<Additive>
        + Ordered<Multiplicative>
{
}
