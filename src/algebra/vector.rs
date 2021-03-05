use crate::algebra::module::Module;
use crate::algebra::ring::Field;

pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field> {
    type Field: Field;
}
