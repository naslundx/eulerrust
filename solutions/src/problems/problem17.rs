use std::collections::HashMap;

pub fn problem17() -> i64 {
    (1..=1000).map(digit_to_text).sum::<i32>() as i64
}

fn digit_to_text(number: i32) -> i32 {
    if number == 0 {
        return 0;
    }

    let data: HashMap<i32, i32> = HashMap::from([
        (1, 3),
        (2, 3),
        (3, 5),
        (4, 4),
        (5, 4),
        (6, 3),
        (7, 5),
        (8, 5),
        (9, 4),
        (10, 3),
        (11, 6),
        (12, 6),
        (13, 8),
        (14, 8),
        (15, 7),
        (16, 7),
        (17, 9),
        (18, 8),
        (19, 8),
        (20, 6),
        (30, 6),
        (40, 5),
        (50, 5),
        (60, 5),
        (70, 7),
        (80, 6),
        (90, 6),
    ]);

    if number < 20 {
        return *data.get(&number).unwrap();
    }
    if number < 100 {
        let tens = (number / 10) * 10;

        let rest = digit_to_text(number - tens);

        let contribution = *data.get(&tens).unwrap();

        return contribution + rest;
    }
    if number < 1000 {
        let hundreds = number / 100;

        let rest = digit_to_text(number - hundreds * 100);

        let contribution = *data.get(&hundreds).unwrap() + 7;

        if rest > 0 {
            return contribution + 3 + rest;
        }

        return contribution;
    }

    11 // onethousand
}
