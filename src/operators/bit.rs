use crate::operators::Operator;
use std::ops::{
    BitAnd as StdBitAnd, BitAndAssign, BitOr as StdBitOr, BitOrAssign, BitXor as StdBitXor,
    BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign,
};

#[derive(Copy, Clone)]
pub struct BitAnd {}

#[derive(Copy, Clone)]
pub struct BitOr {}

#[derive(Copy, Clone)]
pub struct BitXor {}

#[derive(Copy, Clone)]
pub struct BitNot {}

#[derive(Copy, Clone)]
pub struct BitShl {}

#[derive(Copy, Clone)]
pub struct BitShr {}

impl Operator for BitAnd {}
impl Operator for BitOr {}
impl Operator for BitXor {}
impl Operator for BitNot {}
impl Operator for BitShl {}
impl Operator for BitShr {}

pub trait ClosedBitAnd<Rhs = Self>: StdBitAnd<Rhs, Output = Self> + BitAndAssign<Rhs> {}
pub trait ClosedBitOr<Rhs = Self>: StdBitOr<Rhs, Output = Self> + BitOrAssign<Rhs> {}
pub trait ClosedBitXor<Rhs = Self>: StdBitXor<Rhs, Output = Self> + BitXorAssign<Rhs> {}
pub trait ClosedBitShl<Rhs = Self>: Shl<Rhs, Output = Self> + ShlAssign<Rhs> {}
pub trait ClosedBitShr<Rhs = Self>: Shr<Rhs, Output = Self> + ShrAssign<Rhs> {}
pub trait ClosedBitNot: Not<Output = Self> {}

pub trait ClosedBitOps<Rhs = Self>:
    ClosedBitNot
    + ClosedBitAnd<Rhs>
    + ClosedBitOr<Rhs>
    + ClosedBitXor<Rhs>
    + ClosedBitShl<Rhs>
    + ClosedBitShr<Rhs>
{
}

impl<T, Rhs> ClosedBitAnd<Rhs> for T where T: StdBitAnd<Rhs, Output = Self> + BitAndAssign<Rhs> {}
impl<T, Rhs> ClosedBitOr<Rhs> for T where T: StdBitOr<Rhs, Output = Self> + BitOrAssign<Rhs> {}
impl<T, Rhs> ClosedBitXor<Rhs> for T where T: StdBitXor<Rhs, Output = Self> + BitXorAssign<Rhs> {}
impl<T, Rhs> ClosedBitShl<Rhs> for T where T: Shl<Rhs, Output = Self> + ShlAssign<Rhs> {}
impl<T, Rhs> ClosedBitShr<Rhs> for T where T: Shr<Rhs, Output = Self> + ShrAssign<Rhs> {}
impl<T> ClosedBitNot for T where T: Not<Output = Self> {}

impl<T, Rhs> ClosedBitOps<Rhs> for T where
    T: ClosedBitNot
        + ClosedBitAnd<Rhs>
        + ClosedBitOr<Rhs>
        + ClosedBitXor<Rhs>
        + ClosedBitShl<Rhs>
        + ClosedBitShr<Rhs>
{
}
