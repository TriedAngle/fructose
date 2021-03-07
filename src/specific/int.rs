use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{IntegerRing, NaturalCommutativeSemiRing, SemiEuclideanDomain};
use crate::operators::bit::ClosedBitOps;
use crate::operators::{Additive, ClosedOps, Multiplicative};

pub trait Natural: NaturalCommutativeSemiRing + ClosedOps + Zero + One {}

pub trait Integer: IntegerRing + ClosedOps + Zero + One {}

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

impl<T> Natural for T where T: NaturalCommutativeSemiRing + ClosedOps + Zero + One {}

impl<T> Integer for T where T: IntegerRing + ClosedOps + Zero + One {}

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
