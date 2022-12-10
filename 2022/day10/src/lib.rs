use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Self::Noop),
            _ => match s.split_once(' ') {
                Some(("addx", val)) => Ok(Self::Addx(val.parse().map_err(|_e| ())?)),
                _ => Err(()),
            },
        }
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
}

pub fn part1<I: Iterator<Item = Instruction>>(iter: I) -> i64 {
    let states = std::iter::once(1)
        .chain(
            iter.scan(1, |state, instr| {
                Some(match instr {
                    Instruction::Addx(v) => {
                        let prev_state = *state;
                        *state += v;
                        Vec::from([prev_state, *state])
                    }
                    Instruction::Noop => Vec::from([*state]),
                })
            })
            .flatten(),
        )
        .collect::<Vec<_>>();

    for (i, x) in states.iter().copied().enumerate() {
        let sprite_pos = x;
        let beam_pos = (i as i64) % 40;

        if sprite_pos >= beam_pos - 1 && sprite_pos <= beam_pos + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if ((i + 1) % 40) == 0 {
            println!();
        }
    }

    let positions = [20, 60, 100, 140, 180, 220];
    positions
        .iter()
        .map(|p| {
            let state = states[*p - 1]; // why -2??
            state * *p as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT2: &str = include_str!("../example.txt");

    #[test]
    fn test_example_input() {
        assert_eq!(part1(parse_input(EXAMPLE_INPUT2)), 13140);
    }
}
