fn main() {
    let input = include_str!("../input.txt");

    println!("Total priority: {}", find_common_priority(input));
    println!("Group priority sum: {}", elve_groups(input));
}

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common(splits: (&str, &str)) -> Option<char> {
    splits.0.chars().find(|c| splits.1.contains(*c))
}

fn find_common_in_group(group: &[&str]) -> Option<char> {
    group
        .first()?
        .chars()
        .find(|c| group[1..].iter().all(|r| r.contains(*c)))
}

fn item_priority(item: char) -> usize {
    if item.is_ascii_lowercase() {
        ((item as u8) - b'a') as usize + 1
    } else if item.is_ascii_uppercase() {
        ((item as u8) - b'A') as usize + 27
    } else {
        0
    }
}

fn find_common_priority(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| find_common(split_rucksack(line)))
        .map(item_priority)
        .sum()
}

fn elve_groups(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(find_common_in_group)
        .map(item_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_split() {
        let splits = EXAMPLE_INPUT
            .lines()
            .map(split_rucksack)
            .collect::<Vec<_>>();
        assert_eq!(splits[0].0, "vJrwpWtwJgWr");
        assert_eq!(splits[0].1, "hcsFMMfFFhFp");
        assert_eq!(find_common(splits[0]), Some('p'));
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('p'), 16);
        assert_eq!(item_priority('L'), 38);
        assert_eq!(item_priority('P'), 42);
    }

    #[test]
    fn test_example_input() {
        assert_eq!(find_common_priority(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn test_elve_group_sums() {
        assert_eq!(elve_groups(EXAMPLE_INPUT), 70);
    }
}
