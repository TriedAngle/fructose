use crate::algebra::helpers::identity::{One, Zero};
use crate::algebra::operators::{ClosedMul, ClosedSub};
use crate::algebra::properties::euclidean::EuclideanDiv;
use crate::algebra::ring::CommutativeSemiring;

pub fn euclidean<T: CommutativeSemiring + EuclideanDiv + Copy + Zero>(lhs: T, rhs: T) -> T {
    fn helper<U>(a: U, b: U) -> U
    where
        U: CommutativeSemiring + EuclideanDiv + Copy + Zero,
    {
        let r = a.div_euclid_remainder(b.clone());
        if r.is_zero() {
            b
        } else {
            helper(b, r)
        }
    }

    if lhs.is_zero() || rhs.is_zero() {
        return T::zero();
    }

    if lhs.euclid_norm() >= rhs.euclid_norm() {
        helper(lhs, rhs)
    } else {
        helper(rhs, lhs)
    }
}

pub fn extended_euclidean<
    T: CommutativeSemiring + EuclideanDiv + Zero + One + Copy + ClosedMul + ClosedSub,
>(
    lhs: T,
    rhs: T,
) -> (T, T, T) {
    fn helper<U>(a: U, b: U) -> (U, U, U)
    where
        U: CommutativeSemiring + EuclideanDiv + Zero + One + Copy + ClosedMul + ClosedSub,
    {
        let (q, r) = a.div_euclid(b);
        if r.is_zero() {
            (U::zero(), U::one(), b)
        } else {
            let (x1, y1, g) = helper(b, r);
            (y1, x1 - q * y1, g)
        }
    }

    if lhs.is_zero() || rhs.is_zero() {
        return (T::zero(), T::zero(), T::zero());
    }

    if lhs.euclid_norm() >= rhs.euclid_norm() {
        helper(lhs, rhs)
    } else {
        let (y, x, g) = helper(rhs, lhs);
        (x, y, g)
    }
}
