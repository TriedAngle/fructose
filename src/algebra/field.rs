use crate::algebra::lattice::Lattice;
use crate::algebra::ring::DivisionRing;
use crate::operators::{Additive, Multiplicative, Operator};

// (alias) All Fields are Division Rings
pub trait Field<A: Operator = Additive, M: Operator = Multiplicative>: DivisionRing<A, M> {}

pub trait PartiallyOrderedField<A: Operator = Additive, M: Operator = Multiplicative>:
    Field<A, M> + Lattice
{
}

impl<T, A: Operator, M: Operator> Field<A, M> for T where T: DivisionRing<A, M> {}

impl<T, A: Operator, M: Operator> PartiallyOrderedField<A, M> for T where T: Field<A, M> + Lattice {}
