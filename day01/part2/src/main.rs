use std::fs;

fn main() {
    const WINDOW_SIZE: usize = 3;

    let contents = fs::read_to_string("input.txt").expect("Reading input file failed");
    let mut increase_counter = 0;

    let mut ringbuf: [i32; WINDOW_SIZE] = [0; WINDOW_SIZE];
    let mut ringbuf_index: usize = 0;

    let mut last_sum = 0;

    for (i, line) in contents.lines().enumerate() {
        let number: i32 = line.parse().expect("Can't parse line to int");

        ringbuf[ringbuf_index] = number;
        ringbuf_index += 1;
        if ringbuf_index >= WINDOW_SIZE {
            ringbuf_index = 0;
        }

        let sum = ringbuf.iter().sum();

        if i >= 3 && sum > last_sum {
            increase_counter += 1;
        }

        last_sum = sum;
    }

    println!("Depth increased {} times.", increase_counter);
}
