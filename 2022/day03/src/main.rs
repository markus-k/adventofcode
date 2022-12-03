use day03::{elve_groups, find_common_priority};

fn main() {
    let input = include_str!("../input.txt");

    println!("Total priority: {}", find_common_priority(input));
    println!("Group priority sum: {}", elve_groups(input));
}
