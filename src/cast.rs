use std::convert::TryFrom;

pub trait CastInts:
    TryFrom<usize>
    + TryFrom<isize>
    + TryFrom<u8>
    + TryFrom<i8>
    + TryFrom<u16>
    + TryFrom<i16>
    + TryFrom<u32>
    + TryFrom<i32>
    + TryFrom<u64>
    + TryFrom<i64>
    + TryFrom<u128>
    + TryFrom<i128>
{
}

impl<T> CastInts for T where
    T: TryFrom<usize>
        + TryFrom<isize>
        + TryFrom<u8>
        + TryFrom<i8>
        + TryFrom<u16>
        + TryFrom<i16>
        + TryFrom<u32>
        + TryFrom<i32>
        + TryFrom<u64>
        + TryFrom<i64>
        + TryFrom<u128>
        + TryFrom<i128>
{
}

pub trait FromU32 {
    fn from_u32(num: u32) -> Self;
}

pub trait FromI32 {
    fn from_i32(num: i32) -> Self;
}

pub trait FromF32 {
    fn from_f32(num: f32) -> Self;
}

macro_rules! impl_from {
    ($($unsigned:ty:$signed:ty $(:$float:ty)?);* $(;)?) => {
        $(
            impl FromU32 for $unsigned {
                #[inline]
                fn from_u32(num: u32) -> Self {
                    num as Self
                }
            }
            impl FromI32 for $unsigned {
                #[inline]
                fn from_i32(num: i32) -> Self {
                    num.abs() as Self
                }
            }
            impl FromU32 for $signed {
                #[inline]
                fn from_u32(num: u32) -> Self {
                    num as Self
                }
            }
            impl FromI32 for $signed {
                #[inline]
                fn from_i32(num: i32) -> Self {
                    num as Self
                }
            }
            $(
            impl FromU32 for $float {
                #[inline]
                fn from_u32(num: u32) -> Self {
                    num as Self
                }
            }
            impl FromI32 for $float {
                #[inline]
                fn from_i32(num: i32) -> Self {
                    num as Self
                }
            }

            impl FromF32 for $float {
                #[inline]
                fn from_f32(num: f32) -> Self {
                    num as Self
                }
            }
            )?
        )*
    }
}

impl_from! {
    usize:isize;
    u8:i8;
    u16:i16;
    u32:i32:f32;
    u64:i64:f64;
    u128:i128;
}
