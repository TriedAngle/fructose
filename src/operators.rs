use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::process::Output;

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
