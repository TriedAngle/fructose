//!
//! Structure of traits
//!
//! ```rustn't
//!      Totality─────────► Magma───────┬─────┬─────┬──────────┐
//!                           │         │     │     │          ▼
//!                           ▼         │     │     │      Semigroup────────┐
//!               ┌─────►UnitalMagma────┼─────┼─────┼──────┐   ▲            │
//!               │                     │     │     │      │   │            │
//! Associativity─┼─────►Semigroupoid───┼─────┼─────┼──────┼───┘            ▼
//!               │           │         │     │     │      │        InverseSemigroup
//!               │           │         │     ▼     │      ▼                ▲
//!               │           │         │Quasigroup─┼────►Loop              │
//!               │           │         │     ▲     │                       │
//!               │           ▼         │     │     ▼                       │
//!      Identity─┴─────►SmallCategory──┼─────┼─►Monoid──►CommutativeMonoid │
//!                           │         │     │                    ▲        │
//!                           │         │     │                    │        │
//!      Invertibility─┬──────┼─────────┼─────┴────────────────────┼────────┘
//!                    │      │         │                          │
//!                    │      ▼         ▼                          │
//!                    └─►Groupoid───►Group──►AbelianGroup◄───Commutativity
//!
//! ```

use crate::algebra::properties::general::{Associative, Commutative, Identity, Invertible, Total};
use crate::operators::Operator;

pub trait Magma<O: Operator>: Total<O> {}

pub trait UnitalMagma<O: Operator>: Magma<O> + Identity<O> {}

pub trait Semigroupoid<O: Operator>: Associative<O> {}

pub trait SmallCategory<O: Operator>: Semigroupoid<O> + Identity<O> {}

pub trait Groupoid<O: Operator>: SmallCategory<O> + Invertible<O> {}

pub trait Quasigroup<O: Operator>: Magma<O> + Invertible<O> {}

pub trait Loop<O: Operator>: UnitalMagma<O> + Quasigroup<O> {}

pub trait Semigroup<O: Operator>: Magma<O> + Semigroupoid<O> {}

pub trait CommutativeSemigroup<O: Operator>: Semigroup<O> + Commutative<O> {}

pub trait InverseSemigroup<O: Operator>: Semigroup<O> + Invertible<O> {}

pub trait Monoid<O: Operator>: Magma<O> + SmallCategory<O> {}

pub trait CommutativeMonoid<O: Operator>: Monoid<O> + Commutative<O> {}

pub trait Group<O: Operator>: Magma<O> + Groupoid<O> {}

pub trait AbelianGroup<O: Operator>: Group<O> + Commutative<O> {}

impl<T, O: Operator> Magma<O> for T where T: Total<O> {}
impl<T, O: Operator> UnitalMagma<O> for T where T: Magma<O> + Identity<O> {}
impl<T, O: Operator> Semigroupoid<O> for T where T: Associative<O> {}
impl<T, O: Operator> SmallCategory<O> for T where T: Semigroupoid<O> + Identity<O> {}
impl<T, O: Operator> Groupoid<O> for T where T: SmallCategory<O> + Invertible<O> {}

impl<T, O: Operator> Quasigroup<O> for T where T: Magma<O> + Invertible<O> {}
impl<T, O: Operator> Loop<O> for T where T: UnitalMagma<O> + Quasigroup<O> {}
impl<T, O: Operator> Semigroup<O> for T where T: Magma<O> + Semigroupoid<O> {}
impl<T, O: Operator> CommutativeSemigroup<O> for T where T: Semigroup<O> + Commutative<O> {}
impl<T, O: Operator> InverseSemigroup<O> for T where T: Semigroup<O> + Invertible<O> {}
impl<T, O: Operator> Monoid<O> for T where T: Magma<O> + SmallCategory<O> {}
impl<T, O: Operator> CommutativeMonoid<O> for T where T: Monoid<O> + Commutative<O> {}
impl<T, O: Operator> Group<O> for T where T: Magma<O> + Groupoid<O> {}
impl<T, O: Operator> AbelianGroup<O> for T where T: Group<O> + Commutative<O> {}
