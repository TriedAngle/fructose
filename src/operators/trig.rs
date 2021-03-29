use crate::operators::exp::Exponentiation;

pub trait TrigOps: Exponentiation + Sized {
    const PI: Self;
    const TAU: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const FRAC_1_PI: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self {
        self.try_tan().unwrap()
    }
    fn sin_cos(self) -> (Self, Self);

    fn try_tan(self) -> Option<Self>;

    fn asin(self) -> Self {
        self.try_asin().unwrap()
    }
    fn acos(self) -> Self {
        self.try_acos().unwrap()
    }
    fn atan(self) -> Self {
        self.try_atan().unwrap()
    }
    fn atan2(self, rhs: Self) -> Self {
        self.try_atan2(rhs).unwrap()
    }

    fn try_asin(self) -> Option<Self>;
    fn try_acos(self) -> Option<Self>;
    fn try_atan(self) -> Option<Self>;
    fn try_atan2(self, rhs: Self) -> Option<Self>;

    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    fn coth(self) -> Self {
        self.try_coth().unwrap()
    }
    fn csch(self) -> Self {
        self.try_csch().unwrap()
    }

    fn try_coth(self) -> Option<Self>;
    fn try_csch(self) -> Option<Self>;

    fn asinh(self) -> Self;
    fn acosh(self) -> Self {
        self.try_acosh().unwrap()
    }
    fn atanh(self) -> Self {
        self.try_atanh().unwrap()
    }

    fn try_acosh(self) -> Option<Self>;
    fn try_atanh(self) -> Option<Self>;

    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
}

macro_rules! float_to_option {
    ($expr:expr) => {{
        let result = $expr;
        if result.is_infinite() || result.is_nan() {
            None
        } else {
            Some(result)
        }
    }};
}

macro_rules! impl_trig_float {
    ($($set:ty => [$($name: ident = $val:expr);* $(;)?]);* $(;)?) => {
        $(
                impl TrigOps for $set {

                $(
                    const $name: $set = $val;
                )*

                #[inline]
                fn try_tan(self) -> Option<Self> {
                    float_to_option!(self.tan())
                }
                #[inline]
                fn try_asin(self) -> Option<Self> {
                    float_to_option!(self.asin())
                }
                #[inline]
                fn try_acos(self) -> Option<Self> {
                    float_to_option!(self.acos())
                }
                #[inline]
                fn try_atan(self) -> Option<Self> {
                    float_to_option!(self.atan())
                }
                #[inline]
                fn try_atan2(self, rhs: Self) -> Option<Self> {
                    float_to_option!(self.atan2(rhs))
                }
                #[inline]
                fn try_acosh(self) -> Option<Self> {
                    float_to_option!(self.acosh())
                }
                #[inline]
                fn try_atanh(self) -> Option<Self> {
                    float_to_option!(self.atanh())
                }
                #[inline]
                fn try_coth(self) -> Option<Self> {
                    unimplemented!()
                }
                #[inline]
                fn try_csch(self) -> Option<Self> {
                    unimplemented!()
                }
                forward! {
                    fn sin(self) -> Self;
                    fn cos(self) -> Self;
                    fn tan(self) -> Self;
                    fn sin_cos(self) -> (Self, Self);

                    fn asin(self) -> Self;
                    fn acos(self) -> Self;
                    fn atan(self) -> Self;
                    fn atan2(self, rhs: Self) -> Self;

                    fn sinh(self) -> Self;
                    fn cosh(self) -> Self;
                    fn tanh(self) -> Self;

                    fn asinh(self) -> Self;
                    fn acosh(self) -> Self;
                    fn atanh(self) -> Self;

                    fn to_degrees(self) -> Self;
                    fn to_radians(self) -> Self;
                }
            }
        )*
    }
}

impl_trig_float! {
    f32 => [
        PI              = core::f32::consts::PI;
        TAU             = core::f32::consts::TAU;
        FRAC_PI_2       = core::f32::consts::FRAC_PI_2;
        FRAC_PI_3       = core::f32::consts::FRAC_PI_3;
        FRAC_PI_4       = core::f32::consts::FRAC_PI_4;
        FRAC_PI_6       = core::f32::consts::FRAC_PI_6;
        FRAC_PI_8       = core::f32::consts::FRAC_PI_8;
        FRAC_1_PI       = core::f32::consts::FRAC_1_PI;
        FRAC_2_PI       = core::f32::consts::FRAC_2_PI;
        FRAC_2_SQRT_PI  = core::f32::consts::FRAC_2_SQRT_PI;
    ];
    f64 => [
        PI              = core::f64::consts::PI;
        TAU             = core::f64::consts::TAU;
        FRAC_PI_2       = core::f64::consts::FRAC_PI_2;
        FRAC_PI_3       = core::f64::consts::FRAC_PI_3;
        FRAC_PI_4       = core::f64::consts::FRAC_PI_4;
        FRAC_PI_6       = core::f64::consts::FRAC_PI_6;
        FRAC_PI_8       = core::f64::consts::FRAC_PI_8;
        FRAC_1_PI       = core::f64::consts::FRAC_1_PI;
        FRAC_2_PI       = core::f64::consts::FRAC_2_PI;
        FRAC_2_SQRT_PI  = core::f64::consts::FRAC_2_SQRT_PI;
    ];
}
