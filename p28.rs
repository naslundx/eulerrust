fn main() {
    let mut diag_ul_to_dr = vec![1];
    let mut diag_ur_to_dl = vec![1];

    let mut radius = 2;
    let mut r_counter = 0;
    let mut corners = 0;
    let mut idx = 1;

    while idx <= 1001 * 1001 {
        // step
        idx += 1;
        r_counter += 1;

        // reached diagonal
        if r_counter == radius {
            r_counter = 0;
            corners += 1;

            if corners % 2 == 0 {
                diag_ul_to_dr.push(idx);
            } else {
                diag_ur_to_dl.push(idx);
            }

            // completed loop
            if corners % 4 == 0 {
                corners = 0;
                radius += 2;
            }
        }
    }

    println!("ul_to_dr = {:?}", diag_ul_to_dr);
    println!("ur_to_dl = {:?}", diag_ur_to_dl);

    let sum: i64 = diag_ul_to_dr.iter().sum::<i64>() + diag_ur_to_dl.iter().sum::<i64>() - 1;
    println!("sum={sum}");
}
