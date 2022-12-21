#![warn(rust_2018_idioms)]

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!("Invalid operation '{s}'"),
        })
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Add => '+',
                Operation::Subtract => '-',
                Operation::Multiply => '*',
                Operation::Divide => '/',
            }
        )
    }
}

impl Operation {
    pub fn eval<T>(&self, op1: T, op2: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        match self {
            Operation::Add => op1 + op2,
            Operation::Subtract => op1 - op2,
            Operation::Multiply => op1 * op2,
            Operation::Divide => op1 / op2,
        }
    }
}

#[derive(Debug)]
enum Term {
    Formula {
        op: Operation,
        op1: String,
        op2: String,
    },
    Literal(i64),
}

impl FromStr for Term {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Self::Literal(num))
        } else {
            let (op1, rem) = s.split_once(' ').unwrap();
            let (op, op2) = rem.split_once(' ').unwrap();

            Ok(Self::Formula {
                op: op.parse()?,
                op1: op1.to_owned(),
                op2: op2.to_owned(),
            })
        }
    }
}

fn solve(formulae: &HashMap<&str, Term>, name: &str) -> i64 {
    let term = &formulae[name];

    match term {
        Term::Formula { op, op1, op2 } => op.eval(solve(formulae, &op1), solve(formulae, &op2)),
        Term::Literal(literal) => *literal,
    }
}

pub fn part1(input: &str) -> i64 {
    let formulae = input
        .lines()
        .map(|line| {
            let (result_name, formula) = line.split_once(": ").unwrap();
            (result_name, formula.parse::<Term>().unwrap())
        })
        .collect::<HashMap<&str, Term>>();

    solve(&formulae, "root")
}

fn stringify(formulae: &HashMap<&str, Term>, name: &str) -> String {
    let term = &formulae[name];

    if name == "humn" {
        "x".to_owned()
    } else {
        match term {
            Term::Formula { op, op1, op2 } => {
                format!(
                    "({} {} {})",
                    stringify(formulae, &op1),
                    op,
                    stringify(formulae, &op2)
                )
            }
            Term::Literal(literal) => format!("{literal}"),
        }
    }
}

pub fn part2(input: &str) -> String {
    let formulae = input
        .lines()
        .map(|line| {
            let (result_name, formula) = line.split_once(": ").unwrap();
            (result_name, formula.parse::<Term>().unwrap())
        })
        .collect::<HashMap<&str, Term>>();

    if let Term::Formula { op: _, op1, op2 } = &formulae["root"] {
        let formula = format!(
            "{} = {}",
            stringify(&formulae, op1),
            stringify(&formulae, op2)
        );

        // let output = std::process::Command::new("qalc")
        //     .arg("-t")
        //     .arg(formula)
        //     .output()
        //     .expect("Running qalc failed");

        formula
    } else {
        panic!("Expected formula at root");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 152);
    }

    // #[test]
    // fn test_example_input_part2() {
    //     assert_eq!(part2(EXAMPLE_INPUT), 301);
    // }
}
