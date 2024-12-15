fn main() {
    let mut total = 0;
    let mut a = 1;
    let mut b = 1;

    while a + b <= 4000000 {
        let c = a + b;

        if c % 2 == 0 {
            total += c;
        }

        a = b;
        b = c;
    }

    println!("{total}")
}
