// #![allow(dead_code, unused)]

use std::ops::RangeInclusive;

fn main() {
    let range = 1..=5;
    let capacity: i32 = 3;
    let mut combinations: Vec<Vec<i32>> = Vec::new();
    // I think this has to be here instead of the fn for better performance
    let mut previous_nums: Vec<i32> = Vec::new();

    find_combinations(&range, capacity, 0, &mut previous_nums, &mut combinations);

    println!("all subsets of {} numbers from set {:?}:", capacity, range);
    println!("{:?}", combinations);
}

fn find_combinations(
    range: &RangeInclusive<i32>,
    capacity: i32,
    offset: i32,
    previous_nums: &mut Vec<i32>,
    combinations: &mut Vec<Vec<i32>>,
) {
    let range_offset = range.start() + offset..=range.end() - (capacity - 1);

    for num in range_offset {
        previous_nums.push(num);

        if capacity > 1 && num <= range.end() - capacity + 1 {
            find_combinations(
                range,
                capacity - 1,
                num,
                previous_nums,
                combinations,
            );
        } else {
            combinations.push(previous_nums.clone());
        }

        previous_nums.pop();
    }
}

// 0  (min + 0)-(max - (r-1))
// 1  (min + 1)-(max - (r-2))
// 2  (min + 2)-(max - (r-2))
// 12 (min + 2)-(max - (r-3))
// 13 (min + 3)-(max - (r-3))
