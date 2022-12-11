use std::str::FromStr;

#[derive(Debug)]
enum OldOrInt<T> {
    Old,
    Int(T),
}

impl<T> FromStr for OldOrInt<T>
where
    T: FromStr,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            int => Ok(Self::Int(int.parse().map_err(|_| ())?)),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operant2: OldOrInt<u64>,
}

impl Operation {
    fn new(arg: (&str, &str)) -> Self {
        Self {
            operator: arg.0.parse().unwrap(),
            operant2: arg.1.parse().unwrap(),
        }
    }

    fn evaluate(&self, operant1: u64) -> u64 {
        let op2 = match self.operant2 {
            OldOrInt::Old => operant1,
            OldOrInt::Int(int) => int,
        };

        match self.operator {
            Operator::Add => operant1 + op2,
            Operator::Multiply => operant1 * op2,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    test_false: usize,
    test_true: usize,
}

pub fn part1<const DBG: bool>(input: &str) -> usize {
    monkey_in_the_middle::<true, 20, DBG>(input)
}

pub fn part2<const DBG: bool>(input: &str) -> usize {
    monkey_in_the_middle::<false, 10000, DBG>(input)
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter(|monkey| !monkey.is_empty())
        .map(|monkey| {
            let mut lines = monkey.lines().skip(1);
            let items = lines
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            let operation = Operation::new(
                lines
                    .next()
                    .unwrap()
                    .trim_start()
                    .strip_prefix("Operation: new = old ")
                    .unwrap()
                    .split_once(' ')
                    .unwrap(),
            );
            let test_divisor = lines
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let test_true = lines
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let test_false = lines
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test_divisor,
                test_true,
                test_false,
            }
        })
        .collect::<Vec<_>>()
}

macro_rules! maybe_println {
    ($($arg:tt)*) => {
        if DBG { println!($($arg)*) }
    };
}

fn monkey_in_the_middle<const MWL: bool, const R: usize, const DBG: bool>(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let mut inspected_items = vec![0; monkeys.len()];

    // we don't need any number past the modulo of the product of all divisors
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test_divisor)
        .product::<u64>();

    for round in 0..R {
        for i in 0..monkeys.len() {
            maybe_println!("Monkey {i}:");
            let (pre, post) = monkeys.split_at_mut(i);
            let (monkey, post) = post.split_first_mut().unwrap();

            for item in monkey.items.drain(..) {
                maybe_println!("  Monkey inspects an item with a worry level of {item}");
                let mut new_level = monkey.operation.evaluate(item) % modulo;
                maybe_println!("    Worry level is now {new_level}.");
                if MWL {
                    new_level /= 3;
                    maybe_println!(
                        "    Monkey gets bored with item. Worry level is divided by 3 to {new_level}"
                    );
                }
                let new_monkey = if new_level % monkey.test_divisor == 0 {
                    maybe_println!(
                        "    Current worry level is divisible by {}",
                        monkey.test_divisor
                    );
                    maybe_println!(
                        "    Item with worry level {new_level} is thrown to monkey {}",
                        monkey.test_true
                    );
                    monkey.test_true
                } else {
                    maybe_println!(
                        "    Current worry level is not divisible by {}",
                        monkey.test_divisor
                    );
                    maybe_println!(
                        "    Item with worry level {new_level} is thrown to monkey {}.",
                        monkey.test_false
                    );
                    monkey.test_false
                };

                // this weird piece of code avoids mutably aliasing the same monkey twice
                if new_monkey > i {
                    post[new_monkey - i - 1].items.push(new_level);
                } else {
                    pre[new_monkey].items.push(new_level);
                }
                inspected_items[i] += 1;
            }
        }

        if DBG {
            println!("== After round {} ==", round + 1);
            for (i, items) in inspected_items.iter().enumerate() {
                println!("Monkey {i} inspected items {items} times.");
            }
        }
    }

    inspected_items.sort();

    inspected_items[inspected_items.len() - 1] * inspected_items[inspected_items.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(part1::<true>(EXAMPLE_INPUT), 10605);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(part2::<true>(EXAMPLE_INPUT), 2713310158);
    }
}
