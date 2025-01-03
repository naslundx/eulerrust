fn main() {
    let mut primes = Vec::new();
    primes.push(2);
    primes.push(3);

    let mut candidate = 3;

    while primes.len() < 10001 {
        candidate += 1;

        let mut is_prime = true;
        for &p in &primes {
            if candidate % p == 0 {
                is_prime = false;
                break;
            }
        }
        if !is_prime {
            continue;
        }

        primes.push(candidate);
        println!("{candidate}")
    }
}
