use crate::algebra::operators::{Additive, ClosedOps, Multiplicative};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::general::Ordered;
use crate::algebra::properties::primality::Primality;
use crate::algebra::ring::{SemiEuclideanDomain, CommutativeSemiring, CommutativeRing};
use crate::cast::CastInts;

pub trait IntegerSubset:
    CastInts
    + ClosedOps
    + SemiEuclideanDomain
    + Primality
    + ArchimedeanDiv
    + Ordered<Additive>
    + Ordered<Multiplicative>
{
    type Signed: Integer + IntegerSubset<Signed = Self::Signed, Unsigned = Self::Unsigned>;
    type Unsigned: Natural + IntegerSubset<Signed = Self, Unsigned = Self::Unsigned>;
}

pub trait Natural: IntegerSubset<Unsigned = Self> {}

pub trait Integer: IntegerSubset<Signed = Self> + CommutativeRing {}

macro_rules! impl_integer {
    // Entrypoint
    ($($signed:ident:$unsigned:ident), *) => {
        $(
            impl_integer!(@NEXT $unsigned : $signed : $unsigned @natural);
            impl_integer!(@NEXT $signed : $signed : $unsigned @integer);
            impl Natural for $unsigned {}
            impl Integer for $signed {}
        )*
    };
    // Second
    (@NEXT $name:ident : $signed:ident : $unsigned:ident $($tt:tt)*) => {
        impl IntegerSubset for $name {
            type Signed = $signed;
            type Unsigned = $unsigned;
        }
    }
}

impl_integer!(i8:u8, i16:u16, i32:u32, i64:u64, i128:u128, isize:usize);
