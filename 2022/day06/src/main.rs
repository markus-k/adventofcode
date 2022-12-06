use day06::find_start_marker;

fn main() {
    let input = include_str!("../input.txt");

    println!("Start of packet marker: {:?}", find_start_marker(input, 4));
    println!(
        "Start of message marker: {:?}",
        find_start_marker(input, 14)
    );
}
