use day09::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let iter = parse_input(input);
    let visits1 = part1(iter);

    println!("part1: {visits1}");

    let iter = parse_input(input);
    let visits2 = part2(iter);
    println!("part2: {visits2}");
}
