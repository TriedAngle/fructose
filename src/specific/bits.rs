use crate::algebra::ring::EuclideanSemidomain;
use crate::operators::bit::ClosedBitOps;
use crate::operators::ClosedOps;
use crate::properties::archimedean::ArchimedeanDiv;
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
    + Ord
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
        + Ord
{
}
