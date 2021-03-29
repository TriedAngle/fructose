use crate::algebra::ring::EuclideanSemidomain;
use crate::operators::bit::ClosedBitOps;
use crate::operators::{Additive, ClosedOps, Multiplicative};
use crate::properties::archimedean::ArchimedeanDiv;
use crate::properties::general::Ordered;
use crate::properties::helpers::bound::Bounded;
use crate::properties::helpers::identity::{One, Zero};
use crate::properties::primality::Primality;

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
