use crate::algebra::field::Field;
use crate::algebra::helpers::trig::TrigOps;
use crate::algebra::lattice::Lattice;
use crate::algebra::properties::archimedean::ArchimedeanDiv;
use crate::algebra::ring::EuclideanDomain;
use crate::operators::ClosedOps;

pub trait RealField: Field + EuclideanDomain + ArchimedeanDiv + Lattice {}

pub trait ComplexField: Field + EuclideanDomain + ArchimedeanDiv + Lattice {
    type RealField: RealField;
}

pub trait Real: RealField + ClosedOps + TrigOps {}

pub trait Complex: ComplexField + ClosedOps + TrigOps {}
