// #![allow(dead_code, unused)]

use std::{
    collections::HashMap,
    hash::Hash,
    thread,
    time::{Duration, Instant},
};

fn main() {
    compute_heavy_shit();
}

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U: Eq + Hash + Clone, V: Copy> Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn compute_heavy_shit() {
    let mut heavy_shit_u32 = Cacher::new(|num: u32| -> u32 {
        println!("> halt! extensive computation time...");
        thread::sleep(Duration::from_secs(1));
        num * 2
    });

    let mut heavy_shit_str = Cacher::new(|str: &str| -> usize {
        println!("> this is taking so long...");
        thread::sleep(Duration::from_secs(1));
        str.chars().count()
    });

    let strings = vec!["oh", "no", "more", "tests"];

    for attempt in 1..5 {
        let start = Instant::now();
        println!(
            "[attempt {}] u32 -> u32: {}, elapsed: {} ms",
            attempt,
            heavy_shit_u32.value(attempt),
            start.elapsed().as_millis()
        );

        let start = Instant::now();
        println!(
            "[attempt {}] &str -> usize: {}, elapsed: {} ms",
            attempt,
            heavy_shit_str.value(strings[attempt as usize - 1]),
            start.elapsed().as_millis()
        );
    }

    for attempt in (1..5).rev() {
        let start = Instant::now();
        println!(
            "[attempt {}] u32 -> u32 backwards: {}, elapsed: {} ns",
            attempt,
            heavy_shit_u32.value(attempt),
            start.elapsed().as_nanos()
        );

        let start = Instant::now();
        println!(
            "[attempt {}] &str -> usize backwards: {}, elapsed: {} ns",
            attempt,
            heavy_shit_str.value(strings[attempt as usize - 1]),
            start.elapsed().as_nanos()
        )
    }

    let start = Instant::now();
    println!(
        "as compared to not calling value(), elapsed: {} ns",
        start.elapsed().as_nanos()
    );

    let hash_map = HashMap::from([("test", 4), ("blest", 3), ("nest", 6), ("ugabuga", 11)]);

    let start = Instant::now();
    println!(
        "getting {} item from a hashmap: {}, elapsed: {} ns",
        1,
        hash_map.get("test").unwrap(),
        start.elapsed().as_nanos()
    )
}
