use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Reading input file failed");
    let mut last_number = 0;
    let mut increase_counter = 0;

    for (i, line) in contents.lines().enumerate() {
        let number: i32 = line.parse().expect("Can't parse line to int");

        if i > 0 && number > last_number {
            increase_counter += 1;
        }

        last_number = number;
    }

    println!("Depth increased {} times.", increase_counter);
}
