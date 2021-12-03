fn bools_to_int(most_common: &Vec<bool>) -> u64 {
    let value = most_common.iter().fold(0, |acc, &b| acc * 2 + b as u64);

    value
}

fn gamma_rate(most_common: &Vec<bool>) -> u64 {
    bools_to_int(most_common)
}

fn epsilon_rate(most_common: &Vec<bool>) -> u64 {
    let inverted = most_common.iter().map(|b| !b).collect::<Vec<bool>>();

    bools_to_int(&inverted)
}

fn parse_input(input: &str) -> (usize, Vec<bool>) {
    let width = input.lines().next().expect("No first line").len();

    let (total, counters) = input.lines().fold(
        (0, vec![0; width]),
        |(total, counters): (usize, Vec<usize>), line: &str| {
            let new_counters = counters
                .iter()
                .zip(line.chars())
                .map(|(counter, bit)| match bit {
                    '1' => counter + 1,
                    _ => counter + 0,
                })
                .collect::<Vec<usize>>();

            (total + 1, new_counters)
        },
    );

    let most_common = counters.iter().map(|c| *c > total / 2).collect();

    (total, most_common)
}

fn main() {
    let input = include_str!("../input.txt");

    let (_total, most_common) = parse_input(input);

    let gamma = gamma_rate(&most_common);
    let epsilon = epsilon_rate(&most_common);

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Product: {}", gamma * epsilon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let (total, most_common) = parse_input(input);

        assert_eq!(total, 12);

        assert_eq!(gamma_rate(&most_common), 22);
        assert_eq!(epsilon_rate(&most_common), 9);
    }
}
