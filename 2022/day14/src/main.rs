use day14::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");

    let walls = parse_input(input);
    let sand_until_overflow = part1(&walls);
    let sand_until_clogged = part2(&walls);

    println!("part1: {sand_until_overflow}");
    println!("part2: {sand_until_clogged}");
}
