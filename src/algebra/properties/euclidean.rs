use crate::specific::int::Natural;

pub trait EuclideanDiv: Sized {
    type Norm: Natural;

    fn euclid_norm(&self) -> Self::Norm;

    fn div_euclid_quotient(&self, rhs: Self) -> Self {
        let (quot, _) = self.div_euclid(rhs);
        quot
    }

    fn div_euclid_remainder(&self, rhs: Self) -> Self {
        let (_, rem) = self.div_euclid(rhs);
        rem
    }

    fn div_euclid(&self, rhs: Self) -> (Self, Self);
}
