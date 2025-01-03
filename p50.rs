fn main() {
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

    //println!("{:?}", primes);

    let mut best_run = 0;
    let mut best_sum = 0i64;

    let limit = primes.iter().position(|&n| n >= 1000000).unwrap() as usize;

    println!("limit={limit}");
    for start_idx in 0..limit {
        println!("{start_idx}");
        let mut sum = 0i64;
        for end_idx in start_idx..=limit {
            let next_prime = *primes.get(end_idx).expect("exists") as i64;
            sum = sum + next_prime;
            if sum > 1000000 {
                break;
            }
            let run = end_idx - start_idx + 1;
            if run >= best_run && primes.contains(&sum) {
                best_run = run;
                best_sum = sum;
                //println!("sum={sum}, run={run}, p={next_prime} start->end: {start_idx}->{end_idx}");
            }
        }
    }
    println!("best_run={best_run}, best_sum={best_sum}");
}
