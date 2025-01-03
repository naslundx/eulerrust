fn main() {
    let mut primes = std::collections::HashSet::new();
    let mut is_prime = vec![true; 1000001];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=1000000 {
        if is_prime[i] {
            primes.insert(i as i64);
            let mut multiple = i * 2;
            while multiple <= 1000000 {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    let mut n = 7i64;
    loop {
        n += 2;

        if primes.contains(&n) {
            continue;
        }

        let mut s = 1;
        let mut success = false;
        loop {
            let p = n - 2 * s * s;
            if primes.contains(&p) {
                success = true;
                break;
            }
            if p < 2 {
                break;
            }
            s += 1;
        }
        if !success {
            println!("{}", n);
            break;
        }
    }
}
