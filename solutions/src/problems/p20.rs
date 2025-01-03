mod bignumber;

use self::bignumber::BigNumber;

fn main() {
    let mut result = BigNumber::from_number(1);
    for i in 2..=100 {
        result = result.mult(&BigNumber::from_number(i));
    }
    println!("{}", result.digit_sum());
}
