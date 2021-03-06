use crate::algebra::operators::{Additive, Multiplicative, Operator};
use std::ops::{Add, Mul, Neg};

pub trait Set<O: Operator> {
    fn operate(&self, rhs: Self) -> Self;
}

pub trait Associative<O: Operator>: Set<O> {}

pub trait Commutative<O: Operator>: Set<O> {}

// TODO: use this
// TODO: impl this for exponentiation and tetration
pub trait Distributive<O: Operator = Multiplicative, Over: Operator = Additive>:
    Set<O> + Set<Over>
{
}

pub trait Total<O: Operator>: Set<O> {}

pub trait Invertible<O: Operator>: Set<O> + Neg<Output = Self> {
    fn inverse(&self) -> Self;
    fn inverted(&mut self);
}

pub trait Identity<O: Operator>: Set<O> {
    fn identity() -> Self;
    fn is_identity(&self) -> bool;
}

// marker trait for rings without a zero
pub trait NonZero {}

pub trait PartiallyOrdered<O: Operator>: PartialOrd {}
pub trait Ordered<O: Operator>: Ord {}

impl_set! {
    Additive        | add => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Multiplicative  | mul => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
}

impl_properties! {
    Total<Additive>             => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Total<Multiplicative>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Associative<Additive>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Associative<Multiplicative> => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Commutative<Additive>       => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Commutative<Multiplicative> => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;

    PartiallyOrdered<Additive>          => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    PartiallyOrdered<Multiplicative>    => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    Ordered<Additive>                   => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    Ordered<Multiplicative>             => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;

    Distributive<Multiplicative, Additive>  => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;

    NonZero => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
}

impl_identity! {
    0   => Additive         => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    1   => Multiplicative   => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    0.0 => Additive         => f32, f64;
    1.0 => Multiplicative   => f32, f64;
}

impl_invertible_add!(i8 i16 i32 i64 i128 isize f32 f64);
impl_invertible_mul!(f32 f64);
