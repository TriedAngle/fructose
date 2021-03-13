use crate::algebra::group::Monoid;
use crate::algebra::linear::vector::EuclideanSpace;
use crate::operators::Multiplicative;

pub trait Transform<E: EuclideanSpace>: Monoid<Multiplicative> {
    type Translation: Translation<E>;
    type Scaling: Scaling<E>;
    type Rotation: Rotation<E>;

    fn add_translated(&self, translation: &Self::Translation) -> Self;
    fn pre_translated(&self, translation: &Self::Translation) -> Self;

    fn add_scaled(&self, scalar: &Self::Scaling) -> Self;
    fn pre_scaled(&self, scalar: &Self::Scaling) -> Self;

    fn add_rotated(&self, rotation: &Self::Rotation) -> Self;
    fn pre_rotated(&self, rotation: &Self::Rotation) -> Self;

    fn add_translation(&mut self, translation: &Self::Translation);
    fn pre_translation(&mut self, translation: &Self::Translation);

    fn add_scaling(&mut self, scalar: &Self::Scaling);
    fn pre_scaling(&mut self, scalar: &Self::Scaling);

    fn add_rotation(&mut self, rotation: &Self::Rotation);
    fn pre_rotation(&mut self, rotation: &Self::Rotation);
}

pub trait Translation<E: EuclideanSpace>: Transform<E> {
    fn translation(&self) -> Self::Translation;
    fn set_translation(&mut self);
}

pub trait Scaling<E: EuclideanSpace>: Transform<E> {
    fn scaling(&self) -> Self::Scaling;
    fn set_scaling(&mut self);
}

pub trait Rotation<E: EuclideanSpace>: Transform<E> {
    fn rotation(&self) -> Self::Rotation;
    fn set_rotation(&mut self);
}
