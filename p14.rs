use std::collections::HashMap;

fn main() {
    let mut results: HashMap<i64, i64> = HashMap::new();
    
    let mut starting_value = 0;

    loop {
        starting_value += 1;
        let mut current = starting_value;
        let mut steps = 0;

        while current > 1 {
            current = match current % 2 {
                0 => current / 2,
                _ => current * 3 + 1,
            };
            steps += 1;

            if results.contains_key(&current) {
                steps += results.get(&current).unwrap();
                break;
            }
        }

        results.insert(starting_value, steps);

        if starting_value == 1_000_000 {
            break;
        }
    }

    if let Some((&best_key, &best_value)) = results.iter().max_by_key(|&(_, v)| v) {
        println!("{best_key} -> {best_value}");
    }   
}
