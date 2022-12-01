fn main() {
    let input = include_str!("../input.txt");
    let calories = parse_inventories(input);

    let max = max_calories(&calories);
    println!("Biggest inventory: {max}");

    let top3 = top3_calories(&calories);
    println!("Top 3 inventories: {top3}");
}

fn parse_inventories(input: &str) -> Vec<u32> {
    let mut calories = input
        .split("\n\n")
        .map(|inv| inv.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    calories.sort();

    calories
}

fn max_calories(calories: &Vec<u32>) -> u32 {
    *calories.last().unwrap()
}

fn top3_calories(calories: &Vec<u32>) -> u32 {
    calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_example_input_part1() {
        let calories = parse_inventories(EXAMPLE_INPUT);
        assert_eq!(max_calories(&calories), 24000)
    }

    #[test]
    fn test_example_input_part2() {
        let calories = parse_inventories(EXAMPLE_INPUT);
        assert_eq!(top3_calories(&calories), 45000)
    }
}
