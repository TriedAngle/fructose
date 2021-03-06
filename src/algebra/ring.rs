use crate::algebra::group::{AbelianGroup, CommutativeMonoid, Monoid, Semigroup};
use crate::algebra::operators::{Additive, Multiplicative, Operator};

// Addition: Ring
// Multiplication: Ring + Commutativity + Invertibility
pub trait DivisionRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + AbelianGroup<M>
{
}

// Addition: Ring
// Multiplication: Ring + Commutativity
pub trait CommutativeRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + CommutativeMonoid<M>
{
}

// Addition: AbelianGroup
// Multiplication: Associative + Identity => Monoid
pub trait Ring<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + Monoid<M>
{
}

// Addition: Ring - Inverse
// Multiplication: Ring
pub trait Semiring<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeMonoid<A> + Monoid<M>
{
}

// Addition: Ring - Inverse - Commutativity
// Multiplication: Semiring - Identity
pub trait NearRing<A: Operator = Additive, M: Operator = Multiplicative>:
    Monoid<A> + Semigroup<M>
{
}

impl<T, A: Operator, M: Operator> NearRing<A, M> for T where T: Monoid<A> + Semigroup<M> {}
impl<T, A: Operator, M: Operator> Semiring<A, M> for T where T: CommutativeMonoid<A> + Monoid<M> {}
impl<T, A: Operator, M: Operator> Ring<A, M> for T where T: AbelianGroup<A> + Monoid<M> {}
impl<T, A: Operator, M: Operator> CommutativeRing<A, M> for T where
    T: AbelianGroup<A> + CommutativeMonoid<M>
{
}
impl<T, A: Operator, M: Operator> DivisionRing<A, M> for T where T: AbelianGroup<A> + AbelianGroup<M>
{}
