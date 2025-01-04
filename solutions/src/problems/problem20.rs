use big_number::BigNumber;

pub fn problem20() -> i64 {
    let mut result = BigNumber::from_number(1);
    for i in 2..=100 {
        result = result.mult(&BigNumber::from_number(i));
    }
    result.digit_sum() as i64
}
