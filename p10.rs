fn main() {
    let mut primes = Vec::new();
    let mut prime_sum: i64 = 5;

    primes.push(2);
    primes.push(3);

    let mut candidate = 3;

    loop {
        candidate += 2;
        let candidate_sqrt = (candidate as f64).sqrt().ceil() as i64;
        if candidate >= 2000000 {
            break;
        }

        let mut is_prime = true;
        for &p in &primes {
            if candidate % p == 0 {
                is_prime = false;
                break;
            }
            if p > candidate_sqrt {
                break;
            }
        }
        if !is_prime {
            continue;
        }

        primes.push(candidate);
        prime_sum += candidate;
    }
    println!("{prime_sum}")
}
