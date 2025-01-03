use std::collections::HashSet;

fn main() {
    let mut products = HashSet::new();

    for a in 1..10000 {
        for b in a + 1..10000000 {
            let product = a * b;

            let str = a.to_string() + &b.to_string() + &product.to_string();

            if str.len() < 9 {
                continue;
            }
            if str.len() > 9 {
                break;
            }

            if is_pandigital(&str) {
                products.insert(product);
                println!("{}", &product);
            }
        }
    }

    let sum: i32 = products.iter().sum();

    println!("{}", sum);
}

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    for i in 1..=9 {
        if !s.contains(&i.to_string()) {
            return false;
        }
    }
    true
}
