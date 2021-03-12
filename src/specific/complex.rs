use crate::algebra::field::{ComplexField, RealField};
use crate::algebra::helpers::bound::Bounded;
use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::helpers::mul_add::{MulAdd, MulAddAssign};
use crate::algebra::helpers::sign::Signed;

pub trait Real: RealField + Signed + Bounded + MulAdd + MulAddAssign + Zero + One {}

pub trait Complex: ComplexField + Signed + Bounded + Zero + One {}

impl<T> Real for T where T: RealField + Signed + Bounded + MulAdd + MulAddAssign + Zero + One {}

impl<T> Complex for T where T: ComplexField + Signed + Bounded + MulAdd + MulAddAssign + Zero + One {}
