use crate::algebra::helpers::exp::Exponentiation;
use crate::algebra::helpers::trig::TrigOps;
use crate::algebra::lattice::Lattice;
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::ring::{DivisionRing, EuclideanDomain};
use crate::operators::{Additive, ClosedOps, Multiplicative, Operator};

// (alias) All Fields are Division Rings
pub trait Field<A: Operator = Additive, M: Operator = Multiplicative>: DivisionRing<A, M> {}

pub trait PartiallyOrderedField<A: Operator = Additive, M: Operator = Multiplicative>:
    Field<A, M> + Lattice
{
}

pub trait RealField:
    Field + ClosedOps + TrigOps + EuclideanDomain + ArchimedeanDiv + Lattice + Exponentiation
{
}

pub trait ComplexField:
    Field + ClosedOps + TrigOps + EuclideanDomain + ArchimedeanDiv + Lattice + Exponentiation
{
    type RealField: RealField;

    fn from_real(re: Self::RealField) -> Self;

    fn from_imaginary(im: Self::RealField) -> Self;

    fn real(&self) -> Self::RealField;

    fn imaginary(&self) -> Self::RealField;

    fn modulus(&self) -> Self::RealField;

    fn argument(&self) -> Self::RealField;

    fn scale(&self, factor: Self::RealField) -> Self;

    fn unscale(&self, factor: Self::RealField) -> Self;

    fn to_polar(&self) -> (Self::RealField, Self::RealField) {
        (self.modulus(), self.argument())
    }

    fn to_exponential(&self) -> (Self::RealField, Self);

    fn signum(&self) -> Self {
        self.to_exponential().1
    }
}

impl<T, A: Operator, M: Operator> Field<A, M> for T where T: DivisionRing<A, M> {}

impl<T, A: Operator, M: Operator> PartiallyOrderedField<A, M> for T where T: Field<A, M> + Lattice {}

impl<T> RealField for T where
    T: Field + ClosedOps + TrigOps + Exponentiation + EuclideanDomain + ArchimedeanDiv + Lattice
{
}
