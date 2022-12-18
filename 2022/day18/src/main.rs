use day18::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let cubes = parse_input(input);
    let part1 = part1(&cubes);
    println!("part1: {part1}");

    let part2 = part2(&cubes);
    println!("part2: {part2}");
}
