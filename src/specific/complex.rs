use crate::algebra::field::{ComplexField, RealField};
use crate::algebra::helpers::exp::Exponentiation;
use crate::algebra::helpers::trig::TrigOps;

pub trait Real: RealField + TrigOps + Exponentiation {}

pub trait Complex: ComplexField + TrigOps + Exponentiation {}

impl<T> Real for T where T: RealField + TrigOps {}

impl<T> Complex for T where T: ComplexField + TrigOps {}
