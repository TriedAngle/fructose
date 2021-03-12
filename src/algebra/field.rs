use crate::algebra::helpers::exp::Exponentiation;
use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::helpers::trig::TrigOps;
use crate::algebra::lattice::Lattice;
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::ring::{DivisionRing, EuclideanDomain};
use crate::operators::{Additive, ClosedOps, Multiplicative, Operator};

// (alias) All Fields are Division Rings
pub trait Field<A: Operator = Additive, M: Operator = Multiplicative>: DivisionRing<A, M> {}

pub trait PartiallyOrderedField<A: Operator = Additive, M: Operator = Multiplicative>:
    Field<A, M> + Lattice
{
}

pub trait RealField:
    Field + ClosedOps + TrigOps + EuclideanDomain + ArchimedeanDiv + Lattice + Exponentiation
{
}

pub trait ComplexField:
    Field + ClosedOps + TrigOps + EuclideanDomain + ArchimedeanDiv + Lattice + Exponentiation
{
    type RealField: RealField;

    fn from_real(re: Self::RealField) -> Self;

    fn from_imaginary(im: Self::RealField) -> Self;

    fn real(&self) -> Self::RealField;

    fn imaginary(&self) -> Self::RealField;

    fn modulus(&self) -> Self::RealField;

    fn modulus_squared(&self) -> Self::RealField;

    fn argument(&self) -> Self::RealField;

    fn scale(&self, scalar: Self::RealField) -> Self;

    fn unscale(&self, scalar: Self::RealField) -> Self;

    fn to_polar(&self) -> (Self::RealField, Self::RealField) {
        (self.modulus(), self.argument())
    }

    fn to_exponential(&self) -> (Self::RealField, Self);

    fn signum(&self) -> Self {
        self.to_exponential().1
    }
}

impl<T, A: Operator, M: Operator> Field<A, M> for T where T: DivisionRing<A, M> {}

impl<T, A: Operator, M: Operator> PartiallyOrderedField<A, M> for T where T: Field<A, M> + Lattice {}

impl<T> RealField for T where
    T: Field + ClosedOps + TrigOps + Exponentiation + EuclideanDomain + ArchimedeanDiv + Lattice
{
}

macro_rules! impl_complex {
    ($($set:ty)*) => {
        $(
            impl ComplexField for $set {
                type RealField = $set;

                #[inline]
                fn from_real(re: Self::RealField) -> Self {
                    re
                }

                #[inline]
                fn from_imaginary(_: Self::RealField) -> Self {
                    Self::zero()
                }

                #[inline]
                fn real(&self) -> Self::RealField {
                    *self
                }

                #[inline]
                fn imaginary(&self) -> Self::RealField {
                    Self::zero()
                }

                #[inline]
                fn modulus(&self) -> Self::RealField {
                    self.abs()
                }

                #[inline]
                fn modulus_squared(&self) -> Self::RealField {
                    self * self
                }

                #[inline]
                fn argument(&self) -> Self::RealField {
                    if *self >= Self::zero() {
                        Self::zero()
                    } else {
                        Self::PI
                    }
                }

                #[inline]
                fn scale(&self, scalar: Self::RealField) -> Self {
                    self * scalar
                }

                #[inline]
                fn unscale(&self, scalar: Self::RealField) -> Self {
                    self / scalar
                }

                #[inline]
                fn to_exponential(&self) -> (Self::RealField, Self) {
                    let m = self.modulus();

                    if !m.is_zero() {
                        (m, self.unscale(m))
                    } else {
                        (Self::RealField::zero(), Self::one())
                    }
                }
            }
        )*
    }
}

impl_complex!(f32 f64);
