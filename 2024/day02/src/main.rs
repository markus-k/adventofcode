fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|lvl| lvl.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let levels = parse_input(input);

    levels
        .iter()
        .filter(|lvls| {
            lvls.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
                || lvls.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }
}
