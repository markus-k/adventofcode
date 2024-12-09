fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let caps = re.captures_iter(input);

    let mut sum = 0;
    for (_, [a, b]) in caps.map(|c| c.extract()) {
        sum += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
    }

    sum
}

fn part2(input: &str) -> i64 {
    let re = regex::Regex::new(r"((?P<dont>don't\(\))|(?P<do>do\(\))|(?P<mul>mul\((\d+),(\d+)\)))")
        .unwrap();
    let caps = re.captures_iter(input);

    let mut sum = 0;
    let mut en = true;
    for c in caps {
        if c.name("dont").is_some() {
            en = false;
        } else if c.name("do").is_some() {
            en = true;
        } else if c.name("mul").is_some() {
            let a = c.get(5).unwrap().as_str();
            let b = c.get(6).unwrap().as_str();
            if en {
                sum += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT2), 48);
    }
}
