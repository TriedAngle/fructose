use crate::algebra::field::{ComplexField, RealField};
use crate::algebra::module::Module;
use crate::operators::{ClosedAdd, ClosedMul, ClosedOps};
use std::ops::{Neg, Sub};

pub trait Norm {
    type Norm: RealField;
}

pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field> {
    type Field: RealField;
}

pub trait NormedSpace: VectorSpace<Field = <Self as NormedSpace>::ComplexField> + Norm {
    type ComplexField: ComplexField<RealField = Self::Norm>;

    /// essentially magnitude squared
    fn norm_squared(&self) -> Self::Norm;

    /// essentially magnitude
    fn norm(&self) -> Self::Norm;

    fn normalize(&mut self);

    fn normalized(&self) -> Self::Norm;
}

pub trait InnerSpace: NormedSpace {
    fn inner_product(&self, other: &Self) -> Self::ComplexField;

    fn angle(&self, other: &Self) -> Self::Norm;
}

pub trait AffineSpace:
    Sized
    + PartialEq
    + Sub<Self, Output = <Self as AffineSpace>::Translation>
    + ClosedAdd<<Self as AffineSpace>::Translation>
{
    type Translation: VectorSpace;

    fn translate_by(&self, translation: &Self::Translation) -> Self;

    fn subtract(&self, rhs: &Self) -> Self::Translation;
}

pub trait EuclideanSpace:
    AffineSpace<Translation = <Self as EuclideanSpace>::Coordinates>
    + ClosedMul<<Self as EuclideanSpace>::RealField>
    + Neg<Output = Self>
{
    type Coordinates: InnerSpace<ComplexField = Self::RealField, Norm = Self::RealField>
        + ClosedOps<Self::Coordinates>
        + ClosedOps<Self::RealField>;

    type RealField: RealField;

    const ORIGIN: Self;

    fn scale_by(&self, scalar: Self::RealField) -> Self {
        Self::from_coordinates(self.coordinates() * scalar)
    }

    #[inline]
    fn coordinates(&self) -> Self::Coordinates {
        self.subtract(&Self::ORIGIN)
    }

    #[inline]
    fn from_coordinates(coordinates: Self::Coordinates) -> Self {
        Self::ORIGIN.translate_by(&coordinates)
    }

    #[inline]
    fn distance_squared(&self, b: &Self) -> Self::RealField {
        self.subtract(b).norm_squared()
    }

    #[inline]
    fn distance(&self, b: &Self) -> Self::RealField {
        self.subtract(b).norm()
    }
}

macro_rules! impl_vector_space {
    ($($set:ident)*) => {
        $(
            impl Norm for $set {
                type Norm = $set;
            }

            impl VectorSpace for $set {
                type Field = $set;
            }

            impl NormedSpace for $set {
                type ComplexField = $set;

                #[inline]
                fn norm_squared(&self) -> Self::Norm {
                    self.modulus_squared()
                }

                #[inline]
                fn norm(&self) -> Self::Norm {
                    self.modulus()
                }

                #[inline]
                fn normalize(&mut self) {
                    *self /= self.norm();
                }

                #[inline]
                fn normalized(&self) -> Self {
                    *self / self.norm()
                }
            }
        )*
    }
}

impl_vector_space!(f32 f64);
