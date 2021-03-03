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

use crate::properties::{Associative, Identity, Invertible, Total, Commutative};
use crate::operators::Operator;

pub trait Magma<O: Operator>: Total { }

pub trait UnitalMagma<O: Operator>: Magma<O> + Identity<O> { }

pub trait Semigroupoid<O: Operator>: Associative { }

pub trait SmallCategory<O: Operator>: Semigroupoid<O> + Identity<O> { }

pub trait Groupoid<O: Operator>: SmallCategory<O> + Invertible<O> { }

pub trait Quasigroup<O: Operator>: Magma<O> + Invertible<O> { }

pub trait Loop<O: Operator>: UnitalMagma<O> + Quasigroup<O> { }

pub trait Semigroup<O: Operator>: Magma<O> + Semigroupoid<O> { }

pub trait InverseSemigroup<O: Operator>: Semigroup<O> + Invertible<O> { }

pub trait Monoid<O: Operator>: Magma<O> + SmallCategory<O> { }

pub trait CommutativeMonoid<O: Operator>: Monoid<O> + Commutative { }

pub trait Group<O: Operator>: Magma<O> + Groupoid<O> { }

pub trait AbelianGroup<O: Operator>: Group<O> + Commutative { }