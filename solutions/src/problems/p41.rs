use std::collections::HashSet;
use util::is_pandigital_str;

fn main() {
    let mut primes = HashSet::new();
    let limit = 1_000_000_000;
    let mut is_prime = vec![true; limit as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=limit {
        if is_prime[num as usize] {
            primes.insert(num);
            let mut multiple = num * 2;
            while multiple <= limit {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    let mut best_prime = 0;

    for p in primes.iter() {
        //println!("{}", p);
        if *p > best_prime {
            let s = p.to_string();
            if is_pandigital_str(&s) {
                best_prime = *p;
                println!("found: {}", p);
            }
        }
    }

    println!("best: {}", best_prime);
}
