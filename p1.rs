fn main() {
    let mut total = 0;
    let upper_bound = 1000;

    for i in 1..upper_bound {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
        println!("{i}, {total}");
    }
}
