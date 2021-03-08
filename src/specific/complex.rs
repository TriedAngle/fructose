use crate::algebra::field::{ComplexField, RealField};
use crate::algebra::helpers::bound::Bounded;
use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::helpers::mul_add::{MulAdd, MulAddAssign};
use crate::algebra::helpers::sign::Signed;
use crate::algebra::helpers::trig::TrigOps;

pub trait Real:
    RealField + TrigOps + Signed + Bounded + MulAdd + MulAddAssign + Zero + One
{
}

pub trait Complex: ComplexField + TrigOps + Signed + Bounded + Zero + One {}

impl<T> Real for T where
    T: RealField + TrigOps + Signed + Bounded + MulAdd + MulAddAssign + Zero + One
{
}

impl<T> Complex for T where
    T: ComplexField + TrigOps + Signed + Bounded + MulAdd + MulAddAssign + Zero + One
{
}
