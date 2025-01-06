use std::collections::HashSet;
use util::is_pandigital_str;

pub fn problem32() -> i64 {
    let mut products = HashSet::new();

    for a in 1..10000 {
        for b in (a + 1)..10000000 {
            let product = a * b;
            let str = a.to_string() + &b.to_string() + &product.to_string();

            if str.len() < 9 {
                continue;
            }
            if str.len() > 9 {
                break;
            }

            if is_pandigital_str(&str) {
                products.insert(product);
            }
        }
    }

    products.iter().sum::<i64>()
}
