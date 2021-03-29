use crate::algebra::ring::CommutativeSemiring;
use crate::operators::{ClosedMul, ClosedNeg, ClosedOps, ClosedSub};
use crate::properties::euclidean::EuclideanDiv;
use crate::properties::helpers::identity::{One, Zero};

pub fn euclidean<T: CommutativeSemiring + EuclideanDiv + Copy + Zero>(lhs: T, rhs: T) -> T {
    fn helper<U>(a: U, b: U) -> U
    where
        U: CommutativeSemiring + EuclideanDiv + Copy + Zero,
    {
        let r = a.euclid_div_remainder(b.clone());
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
        let (q, r) = a.euclid_div(b);
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

// TODO: use the above algorithm, this is just a copy paste from Glucose
pub fn extended_euclidean_steps<
    T: CommutativeSemiring + EuclideanDiv + Zero + One + Copy + ClosedOps + ClosedNeg,
>(
    mut a: T,
    mut b: T,
) -> (Vec<(T, T)>, Vec<T>, Vec<(T, T)>) {
    assert!(a < b);
    let mut ks = Vec::new();
    let mut abs = vec![(a, b)];
    let mut k;
    let mut a_tmp;
    while b % a != T::zero() {
        k = b / a;
        ks.push(k);
        a_tmp = a;
        a = b - a * k;
        b = a_tmp;
        abs.push((a, b));
    }
    let mut s = T::one();
    let mut t = T::zero();
    let mut sts = vec![(s, t)];
    let mut s_temp;
    let mut ks_clone = ks.clone();
    while let Some(k) = ks_clone.pop() {
        s_temp = s;
        s = k * s * -T::one() + t;
        t = s_temp;
        sts.push((s, t));
    }
    sts.reverse();
    (abs, ks, sts)
}

#[cfg(test)]
mod euclidean_tests {
    use crate::algorithms::euclidean::{extended_euclidean, extended_euclidean_steps};
    use crate::properties::euclidean::EuclideanDiv;

    #[test]
    fn test_euclid_div() {
        let x = 20;
        let y = 6;
        let (a, b) = x.euclid_div(y);
        assert_eq!(a, 3);
        assert_eq!(b, 2);
    }

    #[test]
    fn test_extended_euclid_div() {
        let x = 935;
        let y = 1491;
        let (a, b, _) = extended_euclidean(x, y);
        assert_eq!(a, 716);
        assert_eq!(b, -449);
    }

    #[test]
    fn test_extended_euclid_div_steps() {
        let x = 935;
        let y = 1491;
        let (abs, k, sts) = extended_euclidean_steps(x, y);
        assert_eq!(
            abs,
            vec![
                (935, 1491),
                (556, 935),
                (379, 556),
                (177, 379),
                (25, 177),
                (2, 25),
                (1, 2)
            ]
        );
        assert_eq!(k, vec![1, 1, 1, 2, 7, 12]);
        assert_eq!(
            sts,
            vec![
                (716, -449),
                (-449, 267),
                (267, -182),
                (-182, 85),
                (85, -12),
                (-12, 1),
                (1, 0)
            ]
        );
    }
}
