use crate::module::Module;
use crate::ring::Field;

pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field> {
    type Field: Field;
}
