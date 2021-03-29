use crate::algebra::group::Monoid;
use crate::operators::Operator;

// I don't know... I'd like to use GroupOrder<T = Self, O: Operator> here but this doesn't work?
// => Type parameters with a default must be trailing ?
// no implementation is given as seeing e.g. i32 as infinite or limited is "contextual"
pub trait Orders<T, O: Operator>: Monoid<O> + PartialOrd {
    fn possible_orders() -> Vec<T>;
    fn orders() -> Vec<Option<T>>;
    fn order_of(index: T) -> Option<T>;
    fn producers() -> Vec<T>;
    fn is_producer(&self) -> bool;
    fn order(&self) -> Option<T>;
}
