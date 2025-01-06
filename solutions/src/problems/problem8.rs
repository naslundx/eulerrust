pub fn problem8() -> i64 {
    let content = std::fs::read_to_string("assets/problem8.txt").expect("Failed to read file");
    let content = content.replace("\n", "");

    let mut highest: i64 = 0;
    let mut running_product = 1;
    let mut running_product_contains = 0;
    let series_size = 13;

    for index in 0..content.len() {
        if running_product_contains == series_size && index >= series_size {
            let old_digit = get_digit(&content, index - series_size);
            running_product /= old_digit;
            running_product_contains -= 1;
        }

        let new_digit = get_digit(&content, index);
        running_product *= new_digit;
        running_product_contains += 1;

        if running_product == 0 {
            running_product_contains = 0;
            running_product = 1;
        }

        if running_product > highest {
            highest = running_product;
            println!("{highest}");
        }
    }

    highest
}

fn get_digit(content: &String, index: usize) -> i64 {
    content.chars().nth(index).unwrap().to_digit(10).unwrap() as i64
}
