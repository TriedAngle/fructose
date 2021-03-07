use crate::algebra::field::Field;
use crate::algebra::ring::EuclideanDomain;
use crate::algebra::helpers::trig::TrigOps;
use crate::operators::ClosedOps;

pub trait RealField:
    Field
    + EuclideanDomain
{
}

pub trait ComplexField:
    Field
{
    type RealField: RealField;
}

pub trait Real:
    RealField
    + ClosedOps
    + TrigOps
{
}

pub trait Complex:
    ComplexField
    + ClosedOps
    + TrigOps
{
}