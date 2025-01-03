use std::collections::HashSet;

fn main() {
    let mut primes = HashSet::new();
    let n = 1000000; // You can change this value to any number you want
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
    let mut p = 9;
    while count < 11 {
        p += 2;
        //std::thread::sleep(std::time::Duration::from_millis(100));
        //println!("p={p}");

        if !primes.contains(&p) {
            continue;
        }

        let mut test_from_left = p;
        let mut test_from_right = p;
        let mut candidate_from_left = true;
        while test_from_left > 0 {
            let size = (test_from_left as f64).log10().floor() as u32;
            if size == 0 {
                break;
            }
            let removal = 10i32.pow(size) as u32;
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

    println!("{sum}");
}
