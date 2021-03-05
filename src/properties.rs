use crate::operators::{Additive, Multiplicative, Operator};
use std::ops::Neg;

pub trait Set {}

pub trait Associative<O: Operator>: Set {}

pub trait Commutative<O: Operator>: Set {}

// TODO: use this
pub trait Distributive<O: Operator>: Set {}

pub trait Total<O: Operator>: Set {}

pub trait Invertible<O: Operator>: Set + Neg<Output = Self> {
    fn inverse(&self) -> Self;
    fn inverted(&mut self);
}

pub trait Identity<O: Operator>: Set {
    fn identity() -> Self;
    fn is_identity(&self) -> bool;
}

impl<T> Set for T {}

impl_properties! {
    Total<Additive>             => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Total<Multiplicative>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Associative<Additive>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Associative<Multiplicative> => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Commutative<Additive>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Commutative<Multiplicative> => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
}

impl_identity! {
    0   => Additive         => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    1   => Multiplicative   => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    1.0 => Additive         => f32, f64;
    0.0 => Multiplicative   => f32, f64;
}

impl_invertible_add!(i8 i16 i32 i64 i128 isize f32 f64);
impl_invertible_mul!(f32 f64);
