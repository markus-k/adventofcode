use day05::move_crates;

fn main() {
    let input = include_str!("../input.txt");

    let result = move_crates(input, false);
    println!("Top crates after moving: {:?}", result);

    let result = move_crates(input, true);
    println!("Top crates after moving with cratemover 9001: {:?}", result);
}
