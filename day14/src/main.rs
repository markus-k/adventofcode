use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let polymer = parse_input(input, 10);
    let counts = count_characters(&polymer);
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    println!("{} - {} = {}", max, min, max - min);
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
    println!("{}", polymer);

    polymer
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
    }
}
