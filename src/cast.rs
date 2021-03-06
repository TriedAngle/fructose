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
