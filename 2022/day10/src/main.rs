use day10::{parse_input, part1};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(parse_input(input));
    println!("part1: {part1}");
}
