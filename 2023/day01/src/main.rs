fn main() {
    let input = include_str!("../input.txt");

    println!("part1 = {}", parse_input_part1(input));
    println!("part2 = {}", parse_input_part2(input));
}

fn parse_input_part1(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            if let Some((first, second)) = line
                .chars()
                .find(|c| char::is_numeric(*c))
                .zip(line.chars().rfind(|c| char::is_numeric(*c)))
            {
                (first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap()) as u64
            } else {
                0
            }
        })
        .sum()
}

fn parse_input_part2(input: &str) -> u64 {
    let _digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input.lines().into_iter().map(|_line| 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_INPUT_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(parse_input_part1(EXAMPLE_INPUT_PART1), 142);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(parse_input_part2(EXAMPLE_INPUT_PART2), 281);
    }
}
