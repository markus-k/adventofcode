pub fn find_start_marker(input: &str, length: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_i, window)| {
            window
                .iter()
                .fold(0u32, |acc, c| (acc | (1 << (c - b'a'))))
                .count_ones()
                == length as u32
        })
        .map(|(i, _)| i + length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_inputs() {
        let inputs = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];

        for (input, start_of_packet, start_of_message) in inputs {
            dbg!(input, start_of_packet, start_of_message);
            assert_eq!(find_start_marker(input, 4), Some(start_of_packet));
            assert_eq!(find_start_marker(input, 14), Some(start_of_message));
        }
    }
}
