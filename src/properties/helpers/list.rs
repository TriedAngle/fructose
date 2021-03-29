use crate::properties::helpers::bound::Bounded;
use std::ops::Range;

pub trait ListSet<Rhs = Self>: Sized {
    fn list_set(range: Range<Rhs>) -> Vec<Rhs>;
}

// this trait is not recommended to be used for big sets
pub trait WholeListSet<Rhs = Self>: Bounded<Rhs> {
    fn whole_list_set() -> Vec<Rhs>;
}

macro_rules! impl_list_set {
    ($($set:ty)*) => {
        $(
            impl ListSet for $set {
                #[inline]
                fn list_set(range: Range<$set>) -> Vec<$set> {
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
