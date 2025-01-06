pub fn problem33() -> i64 {
    let mut numerator_product = 1;
    let mut denominator_product = 1;

    for a in 1..=9 {
        for b in 1..=9 {
            for c in a..=9 {
                for d in 1..=9 {
                    let numerator = a * 10 + b;
                    let denominator = c * 10 + d;

                    if numerator >= denominator || (a == b && c == d) {
                        continue;
                    }

                    let fraction = numerator as f64 / denominator as f64;
                    let comparator = if a == c {
                        b as f64 / d as f64
                    } else if b == c {
                        a as f64 / d as f64
                    } else {
                        b as f64 / c as f64
                    };

                    if (fraction - comparator).abs() < 1e-6 {
                        denominator_product *= denominator;
                        numerator_product *= numerator;
                    }
                }
            }
        }
    }

    let mut simplification_idx = 2;
    while simplification_idx <= numerator_product {
        if numerator_product % simplification_idx == 0 && denominator_product % simplification_idx == 0 {
            numerator_product /= simplification_idx;
            denominator_product /= simplification_idx;
        } else {
            simplification_idx += 1;
        }
    }

    denominator_product as i64
}
