use day20::{part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("part1: {part1}");

    let part2 = part2(input);
    println!("part2: {part2}");
}
