use util::get_primes;

pub fn problem10() -> i64 {
    get_primes(200000).iter().take_while(|&&p| p < 2000000).sum()
}
