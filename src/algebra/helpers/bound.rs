pub trait Bounded<Rhs = Self> {
    const MIN: Rhs;
    const MAX: Rhs;
}

macro_rules! impl_bounded {
    ($($set:ty => $min:expr, $max:expr);* $(;)?) => {
        $(
            impl Bounded for $set {
                const MIN: $set = $min;
                const MAX: $set = $max;
            }
        )*
    }
}

impl_bounded! {
    u8  =>  u8::MIN,  u8::MAX;
    u16 => u16::MIN, u16::MAX;
    u32 => u32::MIN, u32::MAX;
    u64 => u64::MIN, u64::MAX;

    i8  =>  i8::MIN,  i8::MAX;
    i16 => i16::MIN, i16::MAX;
    i32 => i32::MIN, i32::MAX;
    i64 => i64::MIN, i64::MAX;

    f32 => f32::MIN, f32::MAX;
    f64 => f64::MIN, f64::MAX;

    u128 => u128::MIN, u128::MAX;
    i128 => i128::MIN, i128::MAX;

    usize => usize::MIN, usize::MAX;
    isize => isize::MIN, isize::MAX;
}
