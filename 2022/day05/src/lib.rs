#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_crates(input: &str) -> Vec<Vec<u8>> {
    let mut stacks = Vec::<Vec<u8>>::with_capacity(9);

    input.lines().rev().skip(1).for_each(|line| {
        line.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| !item.is_ascii_whitespace())
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec = Vec::with_capacity(20);
                    new_vec.push(*item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(*item);
                }
            })
    });

    stacks
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (count, fromto) = line[5..].split_once(" from ")?;
            let (from, to) = fromto.split_once(" to ")?;
            Some(Instruction {
                count: count.parse::<usize>().ok()?,
                from: from.parse::<usize>().ok()?,
                to: to.parse::<usize>().ok()?,
            })
        })
}

fn do_movement(stacks: &mut Vec<Vec<u8>>, instruction: &Instruction, cratemover_9001: bool) {
    let from_stack = stacks.get_mut(instruction.from - 1).unwrap();

    let mut crates = from_stack.split_off(from_stack.len() - instruction.count);

    let to_stack = stacks.get_mut(instruction.to - 1).unwrap();

    if !cratemover_9001 {
        to_stack.extend(crates.iter().rev());
    } else {
        to_stack.append(&mut crates);
    }
}

pub fn move_crates(input: &str, cratemover_9001: bool) -> Vec<u8> {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let stacks = parse_crates(crates);
    let instructions = parse_instructions(instructions);

    let new_stacks = instructions.fold(stacks, |mut stacks, op| {
        do_movement(&mut stacks, &op, cratemover_9001);
        stacks
    });

    new_stacks
        .iter()
        .filter_map(|stack| stack.last().map(|b| *b))
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_example_part1() {
        let top = move_crates(EXAMPLE_INPUT, false);

        assert_eq!(top, b"CMZ");
    }

    #[test]
    fn test_example_part2() {
        let top = move_crates(EXAMPLE_INPUT, true);

        assert_eq!(top, b"MCD");
    }
}
