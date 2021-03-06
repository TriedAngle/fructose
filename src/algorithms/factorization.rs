#[inline]
pub fn wheel_factorization(mut num: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let wheel: [u64; 11] = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
    let (mut f, mut w) = (2, 0);

    while f * f <= num {
        if num % f == 0 {
            primes.push(f);
            num /= f;
        } else {
            f += wheel[w];
            w = if w == 10 { 3 } else { w + 1 };
        }
    }
    primes.push(num);
    primes
}

#[inline]
pub fn is_prime(num: u64) -> bool {
    if num <= 3 && num > 1 {
        true
    } else if num % 2 == 0 || num % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= num {
            if num % i == 0 || num % i + 2 == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}
