use std::ops::RangeInclusive;

fn main() {
    // (1..=2000)
    let list = 1..=5;
}

fn get_all_combinations_of_r_elements(list: &RangeInclusive<i32>, r: i32) -> Vec<Vec<i32>> {
    for n in 1..=r {
        for x in &list[0]..=r {}
    }

    todo!()
}
