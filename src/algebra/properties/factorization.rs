use crate::algorithms::factorization::wheel_factorization;
pub trait Factorizable: Sized {
    type Factors: IntoIterator<Item = Self>;

    fn factors(&self) -> Self::Factors;
}

macro_rules! impl_factorizable {
    ($($set:ty)*) => {
        $(
            impl Factorizable for $set {
                type Factors = Vec<Self>;
                #[inline]
                fn factors(&self) -> Self::Factors {
                    wheel_factorization(*self as u64).iter().map(|e| *e as $set).collect()
                }
            }
        )*
    }
}
impl_factorizable!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
