use std::ops::RangeInclusive;

trait ContainsRange {
    fn fully_contains_range(&self, other: &Self) -> bool;
    fn overlaps_range(&self, other: &Self) -> bool;
}

impl<U> ContainsRange for RangeInclusive<U>
where
    U: PartialOrd<U>,
{
    fn fully_contains_range(&self, other: &Self) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        (other.start() >= self.start() && other.start() <= self.end())
            || (other.end() <= self.end() && other.end() >= self.start())
    }
}

fn parse_input(
    input: &str,
) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input
        .lines()
        .map(|line| line.split(&['-', ',']))
        .filter_map(|mut split| {
            Some((
                split.next()?.parse::<u32>().ok()?..=split.next()?.parse::<u32>().ok()?,
                split.next()?.parse::<u32>().ok()?..=split.next()?.parse::<u32>().ok()?,
            ))
        })
}

pub fn find_fully_contained(input: &str) -> usize {
    parse_input(input)
        .filter(|(first, second)| {
            first.fully_contains_range(second) || second.fully_contains_range(first)
        })
        .count()
}

pub fn find_overlapping(input: &str) -> usize {
    parse_input(input)
        .filter(|(first, second)| first.overlaps_range(second) || second.overlaps_range(first))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(find_fully_contained(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(find_overlapping(EXAMPLE_INPUT), 4);
    }
}
