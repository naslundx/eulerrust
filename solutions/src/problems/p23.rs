use std::collections::HashSet;

fn main() {
    let limit = 28123;
    let mut map: HashSet<i32> = HashSet::new();

    for i in 2..limit {
        if divisor_sum(i) > i {
            map.insert(i);
        }
    }

    println!("done");

    let mut sum = 0;
    for i in 1..=limit {
        if i % 5000 == 0 {
            println!("{}", i);
        }
        let mut candidate = true;
        for j in &map {
            let diff = i - j;
            if diff > 0 && map.contains(&diff) {
                candidate = false;
                break;
            }
        }
        if candidate {
            sum += i;
        }
    }

    println!("{sum}");
}

fn divisor_sum(n: i32) -> i32 {
    let mut sum = 1;
    for i in 2..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}
