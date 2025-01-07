pub fn problem50() -> i64 {
    let mut primes: Vec<i64> = vec![];
    let n = 1100000;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i as i64);
            let mut multiple = i * 2;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    let mut best_run = 0;
    let mut best_sum = 0i64;

    let limit = primes.iter().position(|&n| n >= 1000000).unwrap() as usize;

    for start_idx in 0..limit {
        let mut sum = 0i64;
        for end_idx in start_idx..=limit {
            let next_prime = *primes.get(end_idx).expect("exists") as i64;
            sum += next_prime;
            if sum > 1000000 {
                break;
            }
            let run = end_idx - start_idx + 1;
            if run >= best_run && primes.contains(&sum) {
                best_run = run;
                best_sum = sum;
            }
        }
    }

    best_sum as i64
}
