use big_number::BigNumber;

pub fn problem16() -> i32 {
    let mut number = BigNumber::from_number(1);
    
    for _ in 1..=1000 {
        number = number.mult(&BigNumber::from_number(2));
    }

    number.digit_sum()
}
