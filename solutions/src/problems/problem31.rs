pub fn problem31() -> i64 {
    purchase([0; 8], 0) as i64
}

fn purchase(mut data: [i32; 8], limit: usize) -> i32 {
    let sum = data[0]
        + data[1] * 2
        + data[2] * 5
        + data[3] * 10
        + data[4] * 20
        + data[5] * 50
        + data[6] * 100
        + data[7] * 200;

    if sum > 200 {
        return 0;
    }

    if sum == 200 {
        return 1;
    }

    let mut additions = 0;

    for l in limit..8 {
        data[l] += 1;
        additions += purchase(data, l);
        data[l] -= 1;
    }

    additions
}
