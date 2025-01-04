use big_number::BigNumber;

pub fn problem25() -> i64 {
    let mut idx = 1;
    let mut a = BigNumber::zero();
    let mut b = BigNumber::one();

    while b.digits() < 1000 {
        idx += 1;

        let c = a.add(&b).clone();

        (a, b) = (b, c);
    }

    idx
}
