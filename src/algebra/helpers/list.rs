use std::ops::Range;

pub trait ListSet<Rhs = Self>: Sized {
    fn list_set(range: Range<Rhs>) -> Vec<Self>;
}

macro_rules! impl_list_set {
    ($($set:ty)*) => {
        $(
            impl ListSet for $set {
                #[inline]
                fn list_set(range: Range<$set>) -> Vec<Self> {
                    let mut set = Vec::new();
                    for val in range {
                        set.push(val);
                    }
                    set
                }
            }
        )*
    }
}

impl_list_set!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
