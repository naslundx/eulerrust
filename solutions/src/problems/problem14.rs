use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn problem14() -> i64 {
    let mut results: HashMap<i64, i64> = HashMap::new();
    
    for starting_value in 2..1_000_000 {
        let mut current = starting_value;
        let mut steps = 0;

        while current > 1 {
            current = match current % 2 {
                0 => current / 2,
                _ => current * 3 + 1,
            };
            steps += 1;

            if let Entry::Occupied(v) = results.entry(current) {
                steps += *v.into_mut();
                break;
            }
        }

        results.insert(starting_value, steps);
    }

    *results.iter().max_by_key(|&(_, v)| v).unwrap().0
}
