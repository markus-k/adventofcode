use day19::{parse_input, part1};

fn main() {
    let input = include_str!("../input.txt");
    let blueprints = parse_input(input);
    let part1 = part1(&blueprints);
    println!("part1: {part1}");
}
