use crate::module::Module;
use crate::operators::{Operator, Additive};
use crate::group::AbelianGroup;
use crate::ring::Field;

pub trait VectorSpace<
    A: Operator = Additive,
    FA: Operator = Additive,
    FM: Operator = Additive
>: AbelianGroup<A> + Field<FA, FM>
{
}