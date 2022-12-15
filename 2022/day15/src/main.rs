use day15::{parse_input, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    let sensors = parse_input(input);

    let cant_contain = part1(&sensors, 2000000);
    println!("part1: {cant_contain}");

    let freq = part2(&sensors, (0..=4000000, 0..=4000000));
    println!("part2: {freq}");
}
