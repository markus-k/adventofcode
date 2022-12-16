use day16::{parse_input, part1};

fn main() {
    let input = include_str!("../input.txt");
    let valves = parse_input(input);

    let pr = part1(&valves);
    println!("part1: {pr}");
}
