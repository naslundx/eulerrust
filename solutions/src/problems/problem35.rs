use std::collections::HashSet;

pub fn problem35() -> i64 {
    let mut primes: HashSet<i32> = HashSet::new();
    let n = 1000000;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            primes.insert(i as i32);
            let mut multiple = i * 2;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    let mut counter = 0;

    for n in &primes {
        let mut found = true;
        let mut n_rotated = rotate(*n);

        while *n != n_rotated {
            if n.to_string().len() != n_rotated.to_string().len() || !primes.contains(&n_rotated) {
                found = false;
                break;
            }
            n_rotated = rotate(n_rotated);
        }

        if found {
            counter += 1;
        }
    }

    counter
}

fn rotate(n: i32) -> i32 {
    if n < 10 {
        return n;
    }

    let size = (n as f64).log10().floor() as i32;
    let divisor = 10i32.pow(size as u32);
    let first_digit = n / divisor;

    10 * (n - (divisor * first_digit)) + first_digit
}
