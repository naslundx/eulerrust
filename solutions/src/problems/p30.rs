use std::collections::HashMap;

fn main() {
    let mut fifths: HashMap<u64, u64> = HashMap::new();

    for i in 0..=9 {
        fifths.insert(i, i.pow(5));
    }

    let mut sum = 0;

    for i in 2..400000 {
        if is_match(i, &fifths) {
            println!("{}", i);
            sum += i;
        }
    }

    println!("sum={sum}");
}

fn is_match(n: u64, fifths: &HashMap<u64, u64>) -> bool {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += fifths[&(num % 10)];
        num /= 10;
    }
    sum == n
}
