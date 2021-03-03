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

pub trait Magma<Op: Operator>: Total { }

pub trait UnitalMagma<Op: Operator>: Magma<Op> + Identity<Op> { }

pub trait Semigroupoid<Op: Operator>: Associative { }

pub trait SmallCategory<Op: Operator>: Semigroupoid<Op> + Identity<Op> { }

pub trait Groupoid<Op: Operator>: SmallCategory<Op> + Invertible<Op> { }

pub trait Quasigroup<Op: Operator>: Magma<Op> + Invertible<Op> { }

pub trait Loop<Op: Operator>: UnitalMagma<Op> + Quasigroup<Op> { }

pub trait Semigroup<Op: Operator>: Magma<Op> + Semigroupoid<Op> { }

pub trait InverseSemigroup<Op: Operator>: Semigroup<Op> + Invertible<Op> { }

pub trait Monoid<Op: Operator>: Magma<Op> + SmallCategory<Op> { }

pub trait CommutativeMonoid<Op: Operator>: Monoid<Op> + Commutative { }

pub trait Group<Op: Operator>: Magma<Op> + Groupoid<Op> { }

pub trait AbelianGroup<Op: Operator>: Group<Op> + Commutative { }