fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    input
        .lines()
        .map(|line| {
            let (num1, num2) = line.split_once(' ').unwrap();
            (
                num1.trim_end().parse().unwrap(),
                num2.trim_start().parse().unwrap(),
            )
        })
        .for_each(|(num1, num2)| {
            list1.push(num1);
            list2.push(num2)
        });

    (list1, list2)
}

fn part1(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum()
}

fn part2(input: &str) -> i64 {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .map(|num1| {
            let factor = list2.iter().filter(|&v| v == num1).count() as i64;
            num1 * factor
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_example1() {
        assert_eq!(part1(EXAMPLE_INPUT), 11);
        assert_eq!(part2(EXAMPLE_INPUT), 31);
    }
}
