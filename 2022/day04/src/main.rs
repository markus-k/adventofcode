use day04::{find_fully_contained, find_overlapping};

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "Fully overlapping section ids: {}",
        find_fully_contained(input)
    );
    println!(
        "At all overlapping section ids: {}",
        find_overlapping(input)
    );
}
