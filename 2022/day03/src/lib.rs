fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common(splits: (&str, &str)) -> usize {
    find_common_char(&[splits.0, splits.1])
}

fn find_common_char(strings: &[&str]) -> usize {
    let chars = strings
        .iter()
        .map(|s| {
            s.as_bytes()
                .iter()
                .copied()
                .map(item_priority)
                .fold(0u64, |chars, c| chars | 1 << c)
        })
        .reduce(|acc, chars| acc & chars)
        .unwrap_or(0);

    chars.trailing_zeros() as usize
}

fn item_priority(item: u8) -> u8 {
    (item & 31) + 26 * ((item & 32) == 0) as u8
}

pub fn find_common_priority(input: &str) -> usize {
    input
        .lines()
        .map(|line| find_common(split_rucksack(line)))
        .sum()
}

pub fn elve_groups(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(find_common_char)
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
        //assert_eq!(find_common(splits[0]), Some(b'p'));
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority(b'p'), 16);
        assert_eq!(item_priority(b'L'), 38);
        assert_eq!(item_priority(b'P'), 42);
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
