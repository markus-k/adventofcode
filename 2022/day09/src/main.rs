use console::Term;

use day09::{parse_input, part1, part2_viz, print_rope};

fn main() {
    let input = include_str!("../input.txt");
    let visits1 = part1(parse_input(input));

    println!("part1: {visits1}");

    let mut term = Term::buffered_stdout();

    let visits2 = part2_viz(parse_input(input), |head, tail, tail_positions| {
        term.clear_screen().unwrap();

        print_rope(&mut term, head, tail, tail_positions);

        term.flush().unwrap();
        //std::thread::sleep(std::time::Duration::from_millis(10));
    });
    println!("part2: {visits2}");
}
