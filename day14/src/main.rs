use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let polymer = parse_input(input, 10);
    let counts = count_characters(&polymer);
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    println!("{} - {} = {}", max, min, max - min);

    let count = part2(input, 40);
    println!("After 40 steps: {}", count);
}

fn apply_rules(template: &str, rules: &[(&str, &str)]) -> String {
    (0..(template.len() - 1))
        .map(|i| {
            let pair = &&template[i..i + 2];

            let (_, insertion) = rules.iter().find(|(search, _)| search == pair).unwrap();

            let end = if i == template.len() - 2 {
                &pair[1..2]
            } else {
                ""
            };
            format!("{}{}{}", &pair[0..1], insertion, end)
        })
        .collect::<String>()
}

fn count_characters(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    map
}

fn parse_input(input: &str, steps: usize) -> String {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut polymer: String = template.into();
    for i in 0..steps {
        polymer = apply_rules(&polymer, &rules);
    }

    polymer
}

fn part2(input: &str, steps: usize) -> i64 {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut pairs: HashMap<String, i64> = HashMap::new();
    for i in 0..(template.len() - 1) {
        pairs
            .entry(template[i..i + 2].into())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    for i in 0..steps {
        let mut changes: HashMap<String, i64> = HashMap::new();

        for &(rule, insertee) in rules.iter() {
            if *pairs.entry(rule.into()).or_default() > 0 {
                let count = pairs[rule];
                changes
                    .entry(rule.into())
                    .and_modify(|c| *c -= count)
                    .or_insert(-1 * count);

                let np1: String = format!("{}{}", &rule[0..1], insertee);
                let np2: String = format!("{}{}", insertee, &rule[1..2]);
                changes
                    .entry(np1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                changes
                    .entry(np2)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }

        for (pair, delta) in changes.into_iter() {
            pairs
                .entry(pair)
                .and_modify(|c| *c += delta)
                .or_insert(delta);
        }
    }

    let mut counts: HashMap<char, i64> = HashMap::new();
    for (pair, count) in pairs.iter() {
        for i in [0, 1] {
            let c = pair.chars().nth(i).unwrap();
            counts
                .entry(c)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
    }

    counts = counts
        .iter()
        .map(|(c, count)| (*c, (*count + 1) / 2))
        .collect();

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let polymer = parse_input(input, 10);
        let counts = count_characters(&polymer);
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        println!("{} - {} = {}", max, min, max - min);

        assert_eq!(max - min, 1588);

        assert_eq!(part2(input, 10), 1588);
        assert_eq!(part2(input, 40), 2188189693529);
    }
}
