use crate::algebra::properties::general::{Associative, Ordered};
use crate::operators::Additive;

pub trait ArchimedeanProperty: Ordered<Additive> + Associative<Additive> {}

pub trait ArchimedeanDiv: Sized + ArchimedeanProperty {}

macro_rules! impl_archimedean {
    ($($set:ty)*) => {
        $(
            impl ArchimedeanProperty for $set {}

            impl ArchimedeanDiv for $set {}
        )*
    }
}

impl_archimedean!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
