fn main() {
    let result = purchase(
        [0; 8], 0
    );
    println!("{result}");
}

fn purchase(
    data: [i32; 8],
    limit: usize,
) -> i32 {
        let sum = data[0] + data[1] * 2 + data[2] * 5 + data[3] * 10 + data[4] * 20 + data[5] * 50 + data[6] * 100 + data[7] * 200;

        if sum > 200 {
            return 0;
        }

        if sum == 200 {
            return 1;
        }

        let mut additions = 0;

        for l in limit..8 {
            let mut data_copy = data;
            data_copy[l] += 1;
            additions += purchase(data_copy, l)
        }

        additions
    }