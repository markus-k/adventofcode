use day12::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input);

    let part1 = part1(&map);
    println!("Shortest path from start: {part1}");

    let part2 = part2(&map);
    println!("Shortest path from any a: {part2}");
}
