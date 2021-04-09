use crate::operators::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use crate::properties::helpers::identity::{One, Zero};

pub fn binomial_coefficient<
    T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + One + Zero + Copy,
>(
    n: T,
    mut k: T,
) -> T {
    let mut res = T::one();

    if k > n - T::one() {
        k = n - k
    }
    // TODO: I'd like to use a range T::zero..k here but this requires a bit more of a setup
    let mut counter = T::zero();

    while counter < k {
        res *= n - counter;
        res /= counter + T::one();
        counter += T::one();
    }
    res
}
