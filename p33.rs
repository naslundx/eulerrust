fn main() {
    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                for d in 1..=9 {
                    let numerator = a * 10 + b;
                    let denominator = c * 10 + d;

                    if numerator >= denominator {
                        continue;
                    }

                    if a == b && c == d {
                        continue;
                    }

                    //println!("{}/{}", numerator, denominator);

                    let fraction = numerator as f64 / denominator as f64;
                    let comparator = if a == c {
                        b as f64 / d as f64
                    } else if b == c {
                        a as f64 / d as f64
                    } else {
                        b as f64 / c as f64
                    };

                    if (fraction - comparator).abs() < 1e-6 {
                        println!("-> {}/{}", numerator, denominator);
                    }
                }
            }
        }
    }
}
