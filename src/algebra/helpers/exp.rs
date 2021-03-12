use crate::algebra::field::Field;
use crate::algebra::helpers::pow::PosPow;

pub trait Exponentiation: Field + PosPow + Sized {
    const SQRT_2: Self;
    const FRAC_1_SQRT_2: Self;
    const E: Self;
    const LOG2_E: Self;
    const LOG2_10: Self;
    const LOG10_E: Self;
    const LOG10_2: Self;
    const LN_2: Self;
    const LN_10: Self;

    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp_m1(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
}

macro_rules! impl_exponentiation {
    ($($set:ty)*) => {
        $(
            impl Exponentiation for $set {
                forward! {
                    const SQRT_2: Self;
                    const FRAC_1_SQRT_2: Self;
                    const E: Self;
                    const LOG2_E: Self;
                    const LOG2_10: Self;
                    const LOG10_E: Self;
                    const LOG10_2: Self;
                    const LN_2: Self;
                    const LN_10: Self;
                }

                forward! {
                    fn powi(self, n: i32) -> Self;
                    fn powf(self, n: Self) -> Self;
                    fn sqrt(self) -> Self;
                    fn cbrt(self) -> Self;
                    fn exp(self) -> Self;
                    fn exp_m1(self) -> Self;
                    fn exp2(self) -> Self;
                    fn ln(self) -> Self;
                    fn log(self, base: Self) -> Self;
                    fn log2(self) -> Self;
                    fn log10(self) -> Self;
                }
            }
        )*
    }
}

impl_exponentiation!(f32 f64);
