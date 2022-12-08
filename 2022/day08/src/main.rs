use day08::{best_score, parse_input, visible_trees};

fn main() {
    let input = include_str!("../input.txt");
    let map = parse_input(input);

    println!("Visible trees from outside: {}", visible_trees(&map));
    println!("Best score: {}", best_score(&map));
}
