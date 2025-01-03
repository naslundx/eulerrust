use util::is_palindrome;

pub fn problem4() -> i64 {
    let mut highest = 0;

    for i in 100..1000 {
        for j in i..1000 {
            let product = i * j;
            if is_palindrome(product) && product > highest {
                highest = product;
            }
        }
    }
    
    highest as i64
}


