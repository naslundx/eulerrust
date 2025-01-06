use util::get_primes;

pub fn problem3() -> i64 {
    let n = 600851475143;
    let n_sqrt = (n as f64).sqrt().ceil() as usize;

    let primes = get_primes(n_sqrt / 10);

    for p in primes.iter().rev() {
        if n % p == 0 {
            return *p;
        }
    }

    0
}
