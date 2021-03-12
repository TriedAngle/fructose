use std::num::FpCategory;

pub trait Float {
    const EPSILON: Self;
    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;

    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn classify(self) -> FpCategory;
}

macro_rules! impl_float {
    ($($set:ty)*) => {
        $(
            impl Float for $set {
                forward! {
                    const EPSILON: Self;
                    const MIN: Self;
                    const MIN_POSITIVE: Self;
                    const MAX: Self;
                    const INFINITY: Self;
                    const NEG_INFINITY: Self;
                    const NAN: Self;
                }

                forward! {
                    fn is_infinite(self) -> bool;
                    fn is_nan(self) -> bool;
                    fn is_normal(self) -> bool;
                    fn classify(self) -> FpCategory;
                }
            }
        )*
    }
}

impl_float!(f32 f64);
