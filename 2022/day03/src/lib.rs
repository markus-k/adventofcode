fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common(splits: (&str, &str)) -> Option<char> {
    find_common_char(&[splits.0, splits.1])
}

fn find_common_char(strings: &[&str]) -> Option<char> {
    const OFFSET: u32 = 'A' as u32;

    let chars = strings
        .into_iter()
        .map(|s| {
            s.as_bytes()
                .into_iter()
                .fold(0u64, |chars, c| chars | 1 << (*c as u32 - OFFSET))
        })
        .reduce(|acc, chars| acc & chars)?;

    if chars != 0 {
        char::from_u32(chars.trailing_zeros() + OFFSET as u32)
    } else {
        None
    }
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

pub fn find_common_priority(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| find_common(split_rucksack(line)))
        .map(item_priority)
        .sum()
}

pub fn elve_groups(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(find_common_char)
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
