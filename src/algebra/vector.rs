use crate::algebra::field::Field;
use crate::algebra::module::Module;

pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field> {
    type Field: Field;
}

macro_rules! impl_vector_space {
    ($($set:ident)*) => {
        $(
            impl VectorSpace for $set {
                type Field = $set;
            }
        )*
    }
}

impl_vector_space!(f32 f64);
