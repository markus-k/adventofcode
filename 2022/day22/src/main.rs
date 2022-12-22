use day22::{parse_input, part1};

fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input);
    let part1 = part1(&map);
    println!("part1: {part1}");
}
