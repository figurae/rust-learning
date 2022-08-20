use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 6, 5, 5];

    println!("vector: {:?}", v);
    println!("median: {}", median(&mut v));
    println!("mode: {}", mode(&mut v));
}

fn median(v: &mut [i32]) -> i32 {
    v.sort_unstable();

    let len = v.len();

    println!("sorted vector: {:?}", v);

    match v.len() % 2 {
        0 => (v[len / 2] + v[(len / 2) - 1]) / 2,
        // TODO: this should be a float
        _ => v[(v.len() - 1) / 2],
    }
}

fn mode(v: &mut [i32]) -> i32 {
    let mut mode_map: HashMap<i32, i32> = HashMap::new();

    for item in v {
        let count = mode_map.entry(*item).or_insert(0);
        *count += 1;
    }

    *mode_map.iter().max_by_key(|entry| entry.1).unwrap().0
}
