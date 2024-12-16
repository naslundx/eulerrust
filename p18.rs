use std::fs::read_to_string;

fn main() {
    let filename = String::from("p18data.txt");
    let data: Vec<String> = read_to_string(&filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut array: [i32; 15] = [0; 15];
    let mut size = 0;

    for row in data.iter() {
        size += 1;
        println!("size={size}");

        let leftmost = &row[0..2].parse::<i32>().unwrap();
        array[0] += leftmost;

        for col in 1..size-1 {
            let left = &row[col*3..col*3+2].parse::<i32>().unwrap();
            let right = &row[col*3+3..col*3+5].parse::<i32>().unwrap();
            let largest = std::cmp::max(left, right);
            array[col] += largest;
        }

        if size > 1 {
            let rightmost = &row[(size-1)*3..(size-1)*3+2].parse::<i32>().unwrap();
            println!("rightmost={rightmost}");
            array[size-1] += rightmost;
        }

        println!("{:?}", array);
    }

    println!("{:?}", array);
    println!("{}", array[0]);
}