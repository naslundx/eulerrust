fn main() {
    let mut digits_seen = 0;
    let mut idx = 0;
    let mut looking_for = 1;
    let mut product = 1;

    loop {
        idx += 1;
        digits_seen += digits(idx);

        if digits_seen >= looking_for {
            println!("reached! digits_seen={digits_seen} idx={idx}");

            let mut n = idx;
            for _ in 0..(digits_seen - looking_for) {
                n /= 10;
            }
            let digit = n % 10;
            product *= digit;

            looking_for *= 10;
        }

        if looking_for > 1000000 {
            break;
        }
    }

    println!("{}", product);
}

fn digits(n: i32) -> i32 {
    (n as f64).log10().floor() as i32 + 1
}
