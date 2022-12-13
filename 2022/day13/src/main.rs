use day13::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let signals = parse_input(input);

    println!("part1: {}", part1(&signals));
    println!("part2: {}", part2(&signals));
}
