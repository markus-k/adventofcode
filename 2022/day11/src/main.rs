use day11::{part1, part2};

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1::<true>(input));
    println!("part2: {}", part2::<false>(input));
}
