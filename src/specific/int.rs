use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::euclidean::EuclideanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{SemiEuclideanDomain, EuclideanDomain};
use crate::operators::bit::ClosedBitOps;
use crate::operators::{Additive, ClosedOps, Multiplicative};
use crate::algebra::helpers::identity::{One, Zero};

pub trait NaturalCommutativeSemiRing:
    SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
}

pub trait IntegerRing:
    EuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
}

pub trait Natural:
    NaturalCommutativeSemiRing
    + ClosedOps
    + Zero
    + One
{
}

pub trait Integer:
    IntegerRing
    + ClosedOps
    + Zero
    + One
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

macro_rules! impl_integer {
    // Helpers
    (@UNIT $val:expr => @natural) => {
        $val as Self::Norm
    };
    (@UNIT $val:expr => @integer) => {
        $val.abs() as Self::Norm
    };
    // Entrypoint
    ($($signed:ident:$unsigned:ident), *) => {
        $(
            impl_integer!(@NEXT $unsigned : $signed : $unsigned @natural);
            impl_integer!(@NEXT $signed : $signed : $unsigned @integer);
            impl NaturalCommutativeSemiRing for $unsigned {}
            impl IntegerRing for $signed {}
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
                impl_integer!(@UNIT *self => $($tt)*)
            }

            #[inline]
            fn div_euclid(&self, rhs: Self) -> (Self, Self) {
                (self /rhs, self % rhs)
            }
        }
    }
}

impl_integer!(
    i8    : u8,
    i16   : u16,
    i32   : u32,
    i64   : u64,
    i128  : u128,
    isize : usize
);
