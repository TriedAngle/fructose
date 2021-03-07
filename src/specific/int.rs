use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::euclidean::EuclideanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{CommutativeRing, SemiEuclideanDomain};
use crate::cast::CastInts;
use crate::operators::bit::ClosedBitOps;
use crate::operators::{Additive, ClosedOps, Multiplicative};

pub trait IntegerSubset:
    CastInts
    + ClosedOps
    + SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
    type Unsigned: Natural + IntegerSubset<Signed = Self::Signed, Unsigned = Self::Unsigned>;
    type Signed: Integer + IntegerSubset<Signed = Self::Signed, Unsigned = Self::Unsigned>;
}

pub trait Natural: IntegerSubset<Unsigned = Self> {}

pub trait Integer: IntegerSubset<Signed = Self> + CommutativeRing {}

pub trait Bits: IntegerSubset + ClosedBitOps {}

macro_rules! impl_integer {
    // Entrypoint
    ($($signed:ident:$unsigned:ident), *) => {
        $(
            impl_integer!(@NEXT $unsigned : $signed : $unsigned @natural);
            impl_integer!(@NEXT $signed : $signed : $unsigned @integer);
            impl Natural for $unsigned {}
            impl Integer for $signed {}
            impl Bits for $unsigned {}
            impl Bits for $signed {}
        )*
    };
    // Second
    (@NEXT $set:ident : $signed:ident : $unsigned:ident $($tt:tt)*) => {

        impl EuclideanDiv for $set {
            type Norm = $unsigned;

            #[inline]
            fn euclid_norm(&self) -> Self::Norm {
                *self as Self::Norm
            }

            #[inline]
            fn div_euclid(&self, rhs: Self) -> (Self, Self) {
                (self /rhs, self % rhs)
            }
        }

        impl IntegerSubset for $set {
            type Signed = $signed;
            type Unsigned = $unsigned;
        }
    }
}

impl_integer!(
    i8: u8,
    i16: u16,
    i32: u32,
    i64: u64,
    i128: u128,
    isize: usize
);
