use crate::algorithms::statistics::binomial_coefficient;
use crate::operators::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use crate::properties::helpers::identity::{One, Two, Zero};
use std::fmt::Debug;

pub fn stirling_first<
    T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy,
>(
    n: T,
    k: T,
) -> T {
    fn helper<
        U: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy,
    >(
        n: U,
        k: U,
    ) -> U {
        if n == U::zero() && k == U::zero() {
            U::one()
        } else if k == U::zero() {
            U::zero()
        } else if n == k {
            U::one()
        } else if k == n - U::one() {
            binomial_coefficient(n, U::two())
        } else {
            helper(n - U::one(), k - U::one()) + (n - U::one()) * helper(n - U::one(), k)
        }
    }
    helper(n, k)
}

pub fn stirling_second<
    T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy,
>(
    n: T,
    k: T,
) -> T {
    fn helper<
        U: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy,
    >(
        n: U,
        k: U,
    ) -> U {
        if n == U::zero() && k == U::zero() {
            U::one()
        } else if k == U::zero() {
            U::zero()
        } else if k == U::one() {
            U::one()
        } else if n == k {
            U::one()
        } else if k == n - U::one() {
            binomial_coefficient(n, U::two())
        } else {
            helper(n - U::one(), k - U::one()) + k * helper(n - U::one(), k)
        }
    }
    helper(n, k)
}

pub fn stirling_first_logged<
    T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy + Debug,
>(
    n: T,
    k: T,
) -> (T, Vec<String>) {
    let mut logger = Vec::<String>::new();
    fn helper<
        U: ClosedAdd
            + ClosedSub
            + ClosedMul
            + ClosedDiv
            + PartialOrd
            + Zero
            + One
            + Two
            + Copy
            + Debug,
    >(
        n: U,
        k: U,
        logger: &mut Vec<String>,
    ) -> U {
        if n == U::zero() && k == U::zero() {
            logger.push(format!("P({:?},{:?}) = 1", n, k));
            U::one()
        } else if k == U::zero() {
            logger.push(format!("P({:?},{:?}) = 0", n, k));
            U::zero()
        } else if n == k {
            logger.push(format!("P({:?},{:?}) = 1", n, k));
            U::one()
        } else if k == n - U::one() {
            let tmp_res = binomial_coefficient(n, U::two());
            logger.push(format!("P({:?},{:?}) = binom(n, 2) = {:?}", n, k, tmp_res));
            tmp_res
        } else {
            logger.push(format!(
                "P({:?},{:?}) = P({:?},{:?}) + ({:?} - 1) * P({:?},{:?})",
                n,
                k,
                n - U::one(),
                k - U::one(),
                n,
                n - U::one(),
                k
            ));
            helper(n - U::one(), k - U::one(), logger)
                + (n - U::one()) * helper(n - U::one(), k, logger)
        }
    }
    let res = helper(n, k, &mut logger);
    logger.push(format!("S({:?},{:?}) = {:?}", n, k, res));
    (res, logger)
}

pub fn stirling_second_logged<
    T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv + PartialOrd + Zero + One + Two + Copy + Debug,
>(
    n: T,
    k: T,
) -> (T, Vec<String>) {
    let mut logger = Vec::<String>::new();
    fn helper<
        U: ClosedAdd
            + ClosedSub
            + ClosedMul
            + ClosedDiv
            + PartialOrd
            + Zero
            + One
            + Two
            + Copy
            + Debug,
    >(
        n: U,
        k: U,
        logger: &mut Vec<String>,
    ) -> U {
        if n == U::zero() && k == U::zero() {
            logger.push(format!("S({:?},{:?}) = 1", n, k));
            U::one()
        } else if k == U::zero() {
            logger.push(format!("S({:?},{:?}) = 0", n, k));
            U::zero()
        } else if k == U::one() {
            logger.push(format!("S({:?},{:?}) = 1", n, k));
            U::one()
        } else if n == k {
            logger.push(format!("S({:?},{:?}) = 1", n, k));
            U::one()
        } else if k == n - U::one() {
            let tmp_res = binomial_coefficient(n, U::two());
            logger.push(format!("S({:?},{:?}) = binom(n, 2) = {:?}", n, k, tmp_res));
            tmp_res
        } else {
            logger.push(format!(
                "S({:?},{:?}) = S({:?},{:?}) + {:?} * S({:?},{:?})",
                n,
                k,
                n - U::one(),
                k - U::one(),
                k,
                n - U::one(),
                k
            ));
            helper(n - U::one(), k - U::one(), logger) + k * helper(n - U::one(), k, logger)
        }
    }
    let res = helper(n, k, &mut logger);
    logger.push(format!("S({:?},{:?}) = {:?}", n, k, res));
    (res, logger)
}

#[cfg(test)]
mod stirling_tests {
    use crate::algorithms::stirling::{
        stirling_first, stirling_first_logged, stirling_second, stirling_second_logged,
    };

    #[test]
    fn test_stirling_first() {
        let test_num = stirling_first(4, 2);
        assert_eq!(test_num, 11)
    }

    #[test]
    fn test_stirling_second() {
        let test_num = stirling_second(5, 3);
        assert_eq!(test_num, 25)
    }

    #[test]
    fn test_stirling_first_logged() {
        let (num, logger) = stirling_first_logged(4, 2);
        assert_eq!(num, 11);
        let mut string = String::new();
        for line in logger.iter() {
            string.push_str(line);
        }
        assert_eq!(string, "P(4,2) = P(3,1) + (4 - 1) * P(3,2)P(3,1) = P(2,0) + (3 - 1) * P(2,1)P(2,0) = 0P(2,1) = binom(n, 2) = 1P(3,2) = binom(n, 2) = 3S(4,2) = 11")
    }

    #[test]
    fn test_stirling_second_logged() {
        let (num, logger) = stirling_second_logged(5, 3);
        assert_eq!(num, 25);
        let mut string = String::new();
        for line in logger.iter() {
            string.push_str(line);
        }
        assert_eq!(string, "S(5,3) = S(4,2) + 3 * S(4,3)S(4,2) = S(3,1) + 2 * S(3,2)S(3,1) = 1S(3,2) = binom(n, 2) = 3S(4,3) = binom(n, 2) = 6S(5,3) = 25")
    }
}
