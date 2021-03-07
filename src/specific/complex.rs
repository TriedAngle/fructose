use crate::algebra::field::{ComplexField, RealField};
use crate::algebra::helpers::trig::TrigOps;

pub trait Real: RealField + TrigOps {}

pub trait Complex: ComplexField + TrigOps {}

impl<T> Real for T where T: RealField + TrigOps {}

impl<T> Complex for T where T: ComplexField + TrigOps {}
