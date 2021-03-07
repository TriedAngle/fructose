use crate::algebra::group::{AbelianGroup, CommutativeMonoid, Monoid, Semigroup};
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::properties::bezout::Bezout;
use crate::algebra::properties::euclidean::EuclideanDiv;
use crate::algebra::properties::factorization::Factorizable;
use crate::algebra::properties::gcd::GCD;
use crate::algebra::properties::general::{NonZero, Ordered};
use crate::algebra::properties::primality::Primality;
use crate::operators::{Additive, Multiplicative, Operator};

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

pub trait UFDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    GCDDomain<A, M> + Factorizable
{
}

pub trait PIDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    UFDomain<A, M> + BezoutDomain<A, M>
{
}

pub trait EuclideanDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    PIDomain<A, M> + EuclideanDiv
{
}

// Domains
pub trait SemiDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    Semiring<A, M> + NonZero
{
}

pub trait SemiIntegralDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    CommutativeSemiring<A, M> + NonZero
{
}

pub trait SemiGCDDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    SemiIntegralDomain<A, M> + GCD
{
}

pub trait SemiBezoutDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    SemiGCDDomain<A, M> + Bezout
{
}

pub trait SemiUFDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    SemiGCDDomain<A, M> + Factorizable
{
}

pub trait SemiPIDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    SemiUFDomain<A, M> + SemiBezoutDomain<A, M>
{
}

pub trait SemiEuclideanDomain<A: Operator = Additive, M: Operator = Multiplicative>:
    SemiPIDomain<A, M> + EuclideanDiv
{
}

pub trait NaturalCommutativeSemiRing:
    SemiEuclideanDomain + Primality + ArchimedeanDiv + Ordered<Additive> + Ordered<Multiplicative>
{
}

pub trait IntegerRing:
    EuclideanDomain + Primality + ArchimedeanDiv + Ordered<Additive> + Ordered<Multiplicative>
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
impl<T> UFDomain for T where T: GCDDomain + Factorizable {}
impl<T> PIDomain for T where T: UFDomain + BezoutDomain {}
impl<T> EuclideanDomain for T where T: PIDomain + EuclideanDiv {}

impl<T> SemiDomain for T where T: Semiring + NonZero {}
impl<T> SemiIntegralDomain for T where T: CommutativeSemiring + NonZero {}
impl<T> SemiGCDDomain for T where T: SemiIntegralDomain + GCD {}
impl<T> SemiBezoutDomain for T where T: SemiGCDDomain + Bezout {}
impl<T> SemiUFDomain for T where T: SemiGCDDomain + Factorizable {}
impl<T> SemiPIDomain for T where T: SemiUFDomain + SemiBezoutDomain {}
impl<T> SemiEuclideanDomain for T where T: SemiPIDomain + EuclideanDiv {}

impl<T> NaturalCommutativeSemiRing for T where
    T: SemiEuclideanDomain
        + Primality
        + ArchimedeanDiv
        + Ordered<Additive>
        + Ordered<Multiplicative>
{
}

impl<T> IntegerRing for T where
    T: EuclideanDomain + Primality + ArchimedeanDiv + Ordered<Additive> + Ordered<Multiplicative>
{
}
