use crate::algebra::field::PartiallyOrderedField;
use crate::algebra::helpers::sign::Signed;
use crate::algebra::helpers::trig::TrigOps;
use crate::operators::ClosedOps;

pub trait RealField: PartiallyOrderedField + ClosedOps + Copy + Signed + TrigOps {
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
}

macro_rules! impl_real_field {
    ($($set:ident)*) => {
        $(
            impl RealField for $set {
                forward! {
                    fn powi(self, n: i32) -> Self;
                    fn powf(self, n: Self) -> Self;
                }
            }
        )*
    }
}

impl_real_field!(f32 f64);
