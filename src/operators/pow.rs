pub trait PosPow<Rhs = u32> {
    type Output;

    fn pow(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_pow {
    ($($set:ty => [$method:expr => $from:ty]);+) => {
        $(
            impl PosPow<$set> for $set {
                type Output = $set;

                #[inline]
                fn pow(self, rhs: $set) -> Self::Output {
                    ($method)(self, rhs as $from)
                }
            }
        )*
    }
}

impl_pow! {
    u8      => [u8::pow     => u32];
    u16     => [u16::pow    => u32];
    u32     => [u32::pow    => u32];
    u64     => [u64::pow    => u32];
    i8      => [i8::pow     => u32];
    i16     => [i16::pow    => u32];
    i32     => [i32::pow    => u32];
    i64     => [i64::pow    => u32];
    u128    => [u128::pow   => u32];
    i128    => [i128::pow   => u32];
    usize   => [usize::pow  => u32];
    isize   => [isize::pow  => u32];
    f32     => [f32::powi   => i32];
    f64     => [f64::powi   => i32]
}
