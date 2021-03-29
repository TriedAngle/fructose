use crate::operators::pow::PosPow;

pub trait Exponentiation: PosPow + Sized {
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
    ($($set:ty => [$($name: ident = $val:expr);* $(;)?]);* $(;)?) => {
        $(
            impl Exponentiation for $set {
                $(
                    const $name: $set = $val;
                )*

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

impl_exponentiation! {
    f32 => [
        SQRT_2          = core::f32::consts::SQRT_2;
        FRAC_1_SQRT_2   = core::f32::consts::FRAC_1_SQRT_2;
        E               = core::f32::consts::E;
        LOG2_E          = core::f32::consts::LOG2_E;
        LOG2_10         = core::f32::consts::LOG2_10;
        LOG10_E         = core::f32::consts::LOG10_E;
        LOG10_2         = core::f32::consts::LOG10_2;
        LN_2            = core::f32::consts::LN_2;
        LN_10           = core::f32::consts::LN_10;
    ];
    f64 => [
        SQRT_2          = core::f64::consts::SQRT_2;
        FRAC_1_SQRT_2   = core::f64::consts::FRAC_1_SQRT_2;
        E               = core::f64::consts::E;
        LOG2_E          = core::f64::consts::LOG2_E;
        LOG2_10         = core::f64::consts::LOG2_10;
        LOG10_E         = core::f64::consts::LOG10_E;
        LOG10_2         = core::f64::consts::LOG10_2;
        LN_2            = core::f64::consts::LN_2;
        LN_10           = core::f64::consts::LN_10;
    ];
}
