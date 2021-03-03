use crate::group::{AbelianGroup, CommutativeMonoid, Group, Monoid, Semigroup};
use crate::operators::{Additive, Multiplicative, Operator};

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
    Group<A> + Semigroup<Multiplicative>
{
}
