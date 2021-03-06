use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Operator: Copy {}

#[derive(Copy, Clone)]
pub struct Additive {}

#[derive(Copy, Clone)]
pub struct Multiplicative {}

impl Operator for Additive {}

impl Operator for Multiplicative {}

pub trait ClosedAdd<Rhs = Self>: Add<Rhs, Output = Self> + AddAssign<Rhs> {}
pub trait ClosedSub<Rhs = Self>: Sub<Rhs, Output = Self> + SubAssign<Rhs> {}
pub trait ClosedMul<Rhs = Self>: Mul<Rhs, Output = Self> + MulAssign<Rhs> {}
pub trait ClosedDiv<Rhs = Self>: Div<Rhs, Output = Self> + DivAssign<Rhs> {}
pub trait ClosedNeg: Neg<Output = Self> {}

pub trait ClosedOps<Rhs = Self>:
    ClosedAdd<Rhs> + ClosedSub<Rhs> + ClosedMul<Rhs> + ClosedDiv<Rhs>
{
}

impl<T, Rhs> ClosedAdd<Rhs> for T where T: Add<Rhs, Output = T> + AddAssign<Rhs> {}
impl<T, Rhs> ClosedSub<Rhs> for T where T: Sub<Rhs, Output = T> + SubAssign<Rhs> {}
impl<T, Rhs> ClosedMul<Rhs> for T where T: Mul<Rhs, Output = T> + MulAssign<Rhs> {}
impl<T, Rhs> ClosedDiv<Rhs> for T where T: Div<Rhs, Output = T> + DivAssign<Rhs> {}
impl<T> ClosedNeg for T where T: Neg<Output = T> {}
impl<T, Rhs> ClosedOps<Rhs> for T where
    T: ClosedAdd<Rhs> + ClosedSub<Rhs> + ClosedMul<Rhs> + ClosedDiv<Rhs>
{
}
