fn main() {
    let contents = include_str!("../input.txt");
    let increase_counter = count_increases_with_moving_average(contents);

    println!("Depth increased {} times.", increase_counter);
}

fn count_increases_with_moving_average(contents: &str) -> usize {
    const WINDOW_SIZE: usize = 3;

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

        if i >= WINDOW_SIZE && sum > last_sum {
            increase_counter += 1;
        }

        last_sum = sum;
    }

    return increase_counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_correctly() {
        let input = "607
618
618
617
647
716
769
792";
        let count = count_increases_with_moving_average(input);

        assert_eq!(count, 5);
    }
}
