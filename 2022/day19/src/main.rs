use day19::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let blueprints = parse_input(input);
    let part1 = part1(&blueprints);
    println!("part1: {part1}");

    let part2 = part2(&blueprints);
    println!("part2: {part2}");
}
