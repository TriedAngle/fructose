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

pub trait RealField: Field + ClosedOps + EuclideanDomain + ArchimedeanDiv + Lattice {}

pub trait ComplexField: Field + ClosedOps + EuclideanDomain + ArchimedeanDiv + Lattice {
    type RealField: RealField;
}

impl<T, A: Operator, M: Operator> Field<A, M> for T where T: DivisionRing<A, M> {}

impl<T, A: Operator, M: Operator> PartiallyOrderedField<A, M> for T where T: Field<A, M> + Lattice {}

impl<T> RealField for T where T: Field + ClosedOps + EuclideanDomain + ArchimedeanDiv + Lattice {}

impl<T> ComplexField for T
where
    T: Field + ClosedOps + EuclideanDomain + ArchimedeanDiv + Lattice,
{
    type RealField = T;
}
