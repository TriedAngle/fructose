use crate::algebra::group::{AbelianGroup, CommutativeMonoid, Monoid, Semigroup};
use crate::operators::{
    Additive, ClosedAdd, ClosedMul, ClosedNeg, ClosedRem, ClosedSub, Multiplicative, Operator,
};
use crate::properties::archimedean::ArchimedeanDiv;
use crate::properties::bezout::Bezout;
use crate::properties::euclidean::EuclideanDiv;
use crate::properties::factorization::Factorizable;
use crate::properties::gcd::GCD;
use crate::properties::general::NonZero;
use crate::properties::primality::Primality;

// Rings

/// Addition: Ring
/// Multiplication: Ring + Commutativity + Invertibility
pub trait DivisionRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + AbelianGroup<M>
{
}

/// Addition: Ring
/// Multiplication: Ring + Commutativity
pub trait CommutativeRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + CommutativeMonoid<M>
{
}

/// Addition: AbelianGroup
/// Multiplication: Associative + Identity => Monoid
pub trait Ring<A: Operator = Additive, M: Operator = Multiplicative>:
    AbelianGroup<A> + Monoid<M>
{
}

/// Addition: Ring - Inverse
/// Multiplication: Ring
pub trait Semiring<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeMonoid<A> + Monoid<M>
{
}

pub trait CommutativeSemiring<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeMonoid<A> + CommutativeMonoid<M>
{
}

/// Addition: Ring - Inverse - Commutativity
/// Multiplication: Semiring - Identity
pub trait NearRing<A: Operator = Additive, M: Operator = Multiplicative>:
    Monoid<A> + Semigroup<M>
{
}

// Domains
pub trait Domain<A: Operator = Additive, M: Operator = Multiplicative>:
    Ring<A, M> + NonZero
{
}

pub trait IntegralDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeRing<A, M> + NonZero
{
}

pub trait GCDDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    IntegralDomain<A, M> + GCD
{
}

pub trait BezoutDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    GCDDomain<A, M> + Bezout
{
}

pub trait UFDDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    GCDDomain<A, M> + Factorizable
{
}

pub trait PIDDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    UFDDomain<A, M> + BezoutDomain<A, M>
{
}

pub trait EuclideanDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    PIDDomain<A, M> + EuclideanDiv
{
}

// Domains
pub trait Semidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    Semiring<A, M> + NonZero
{
}

pub trait IntegralSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeSemiring<A, M> + NonZero
{
}

pub trait GCDSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    IntegralSemidomain<A, M> + GCD
{
}

pub trait BezoutSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    GCDSemidomain<A, M> + Bezout
{
}

pub trait UFDSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    GCDSemidomain<A, M> + Factorizable
{
}

pub trait PIDSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    UFDSemidomain<A, M> + BezoutSemidomain<A, M>
{
}

pub trait EuclideanSemidomain<A: Operator = Additive, M: Operator = Multiplicative>:
    PIDSemidomain<A, M> + EuclideanDiv
{
}

pub trait NaturalCommutativeSemiring:
    EuclideanSemidomain + ClosedAdd + ClosedMul + ClosedRem + Primality + ArchimedeanDiv + Eq + Ord
{
}

pub trait IntegerRing:
    EuclideanDomain
    + ClosedAdd
    + ClosedSub
    + ClosedMul
    + ClosedRem
    + ClosedNeg
    + Primality
    + ArchimedeanDiv
    + Eq
    + Ord
{
}

impl<T> NearRing for T where T: Monoid<Additive> + Semigroup<Multiplicative> {}

impl<T> Semiring for T where T: CommutativeMonoid<Additive> + Monoid<Multiplicative> {}

impl<T> CommutativeSemiring for T where
    T: CommutativeMonoid<Additive> + CommutativeMonoid<Multiplicative>
{
}

impl<T> Ring for T where T: AbelianGroup<Additive> + Monoid<Multiplicative> {}

impl<T> CommutativeRing for T where T: AbelianGroup<Additive> + CommutativeMonoid<Multiplicative> {}

impl<T> DivisionRing for T where T: AbelianGroup<Additive> + AbelianGroup<Multiplicative> {}

impl<T> Domain for T where T: Ring + NonZero {}
impl<T> IntegralDomain for T where T: CommutativeRing + NonZero {}
impl<T> GCDDomain for T where T: IntegralDomain + GCD {}
impl<T> BezoutDomain for T where T: GCDDomain + Bezout {}
impl<T> UFDDomain for T where T: GCDDomain + Factorizable {}
impl<T> PIDDomain for T where T: UFDDomain + BezoutDomain {}
impl<T> EuclideanDomain for T where T: PIDDomain + EuclideanDiv {}

impl<T> Semidomain for T where T: Semiring + NonZero {}
impl<T> IntegralSemidomain for T where T: CommutativeSemiring + NonZero {}
impl<T> GCDSemidomain for T where T: IntegralSemidomain + GCD {}
impl<T> BezoutSemidomain for T where T: GCDSemidomain + Bezout {}
impl<T> UFDSemidomain for T where T: GCDSemidomain + Factorizable {}
impl<T> PIDSemidomain for T where T: UFDSemidomain + BezoutSemidomain {}
impl<T> EuclideanSemidomain for T where T: PIDSemidomain + EuclideanDiv {}

impl<T> NaturalCommutativeSemiring for T where
    T: EuclideanSemidomain
        + ClosedAdd
        + ClosedMul
        + ClosedRem
        + Primality
        + ArchimedeanDiv
        + Eq
        + Ord
{
}

impl<T> IntegerRing for T where
    T: EuclideanDomain
        + ClosedAdd
        + ClosedSub
        + ClosedMul
        + ClosedRem
        + ClosedNeg
        + Primality
        + ArchimedeanDiv
        + Eq
        + Ord
{
}
