pub fn problem43() -> i64 {
    let mut sum = 0i64;
    for d_1 in 0..10 {
        for d_2 in 0..10 {
            if d_2 != d_1 {
                for d_3 in 0..10 {
                    if d_3 != d_2 && d_3 != d_1 {
                        for d_4 in [0, 2, 4, 6, 8] {
                            if d_4 != d_3 && d_4 != d_2 && d_4 != d_1 {
                                for d_5 in 0..10 {
                                    if d_5 != d_4
                                        && d_5 != d_3
                                        && d_5 != d_2
                                        && d_5 != d_1
                                        && check_div(d_3, d_4, d_5, 3)
                                    {
                                        for d_6 in [0, 5] {
                                            if d_6 != d_5
                                                && d_6 != d_4
                                                && d_6 != d_3
                                                && d_6 != d_2
                                                && d_6 != d_1
                                            {
                                                for d_7 in 0..10 {
                                                    if d_7 != d_6
                                                        && d_7 != d_5
                                                        && d_7 != d_4
                                                        && d_7 != d_3
                                                        && d_7 != d_2
                                                        && d_7 != d_1
                                                        && check_div(d_5, d_6, d_7, 7)
                                                    {
                                                        for d_8 in 0..10 {
                                                            if d_8 != d_7
                                                                && d_8 != d_6
                                                                && d_8 != d_5
                                                                && d_8 != d_4
                                                                && d_8 != d_3
                                                                && d_8 != d_2
                                                                && d_8 != d_1
                                                                && check_div(d_6, d_7, d_8, 11)
                                                            {
                                                                for d_9 in 0..10 {
                                                                    if d_9 != d_8
                                                                        && d_9 != d_7
                                                                        && d_9 != d_6
                                                                        && d_9 != d_5
                                                                        && d_9 != d_4
                                                                        && d_9 != d_3
                                                                        && d_9 != d_2
                                                                        && d_9 != d_1
                                                                        && check_div(
                                                                            d_7, d_8, d_9, 13,
                                                                        )
                                                                    {
                                                                        for d_10 in 0..10 {
                                                                            if d_10 != d_9
                                                                                && d_10 != d_8
                                                                                && d_10 != d_7
                                                                                && d_10 != d_6
                                                                                && d_10 != d_5
                                                                                && d_10 != d_4
                                                                                && d_10 != d_3
                                                                                && d_10 != d_2
                                                                                && d_10 != d_1
                                                                                && check_div(
                                                                                    d_8, d_9, d_10,
                                                                                    17,
                                                                                )
                                                                            {
                                                                                let n: i64 = [
                                                                                    d_1, d_2, d_3,
                                                                                    d_4, d_5, d_6,
                                                                                    d_7, d_8, d_9,
                                                                                    d_10,
                                                                                ]
                                                                                .iter()
                                                                                .fold(
                                                                                    0i64,
                                                                                    |acc, &x| {
                                                                                        acc * 10 + x as i64
                                                                                    },
                                                                                );
                                                                                sum += n;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

fn check_div(n1: i32, n2: i32, n3: i32, d: i32) -> bool {
    let s = n1 * 100 + n2 * 10 + n3;
    s % d == 0
}
