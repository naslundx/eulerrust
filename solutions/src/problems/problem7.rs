use util::get_primes;

pub fn problem7() -> i64 {
    let primes = get_primes(10001);
    *primes.last().unwrap()
}
