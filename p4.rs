fn main() {
    let mut highest = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let product = i * j;
            if is_palindrome(product) && product > highest {
                println!("{i} * {j} = {product}");
                highest = product;
            }
        }
    }
}

fn is_palindrome(n: i32) -> bool {
    let mut divisor_left = 10f64.powf((n as f64).log10().floor()) as i32;
    let mut divisor_right = 1;

    while divisor_left > divisor_right {
        let digit_left = (n / divisor_left) % 10;
        let digit_right = (n / divisor_right) % 10;

        if digit_left != digit_right {
            return false;
        }

        divisor_left /= 10;
        divisor_right *= 10;
    }

    true
}
