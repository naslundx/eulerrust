use std::collections::HashSet;

pub fn problem37() -> i64 {
    let mut primes = HashSet::new();
    let n = 1000000;
    for num in 2..=n {
        let mut is_prime = true;
        for i in 2..=((num as f64).sqrt() as u32) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.insert(num);
        }
    }

    let mut sum = 0;
    let mut count = 0;
    for p in primes.iter().filter(|&&n| n > 10) {
        if count > 11 {
            break;
        }

        let mut test_from_left = *p;
        let mut test_from_right = *p;
        let mut candidate_from_left = true;

        while test_from_left > 0 {
            let size = (test_from_left as f64).log10().floor() as u32;
            if size == 0 {
                break;
            }
            let removal = 10u32.pow(size);
            test_from_left = test_from_left % removal;
            test_from_right = test_from_right / 10;

            if !primes.contains(&test_from_left) || !primes.contains(&test_from_right) {
                candidate_from_left = false;
                break;
            }
        }

        if candidate_from_left {
            sum += p;
            count += 1;
        }
    }

    sum as i64
}
