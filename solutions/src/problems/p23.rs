use std::collections::HashSet;
use utils::divisor_sum;

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
