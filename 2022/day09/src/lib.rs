use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(dir, steps)| {
            Some((dir.parse::<Direction>().ok()?, steps.parse::<usize>().ok()?))
        })
        .flat_map(|(dir, steps)| std::iter::repeat(dir).take(steps))
}

fn step_head(head: (isize, isize), dir: Direction) -> (isize, isize) {
    let step = dir.offset();
    (head.0 + step.0, head.1 + step.1)
}

fn step_tail(head: (isize, isize), mut tail: (isize, isize)) -> (isize, isize) {
    let distance = (head.0 - tail.0, head.1 - tail.1);
    match distance {
        (-2, 0) | (2, 0) | (0, -2) | (0, 2) => {
            tail = (tail.0 + distance.0 / 2, tail.1 + distance.1 / 2)
        }
        (-1, -1) | (-1, 1) | (1, -1) | (1, 1) => {} // ok
        (-1, 0) | (1, 0) | (0, -1) | (0, 1) => {}   // ok
        (0, 0) => {}                                // overlap is allowed
        _ => {
            // move one step diagonally towards head
            tail = (
                tail.0 + distance.0 / distance.0.abs(),
                tail.1 + distance.1 / distance.1.abs(),
            )
        }
    }

    tail
}

pub fn part1<I: Iterator<Item = Direction>>(iter: I) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::<(isize, isize)>::new();

    for dir in iter {
        head = step_head(head, dir);
        tail = step_tail(head, tail);

        tail_positions.insert(tail);
    }

    //print_tail(&tail_positions);

    tail_positions.len()
}

pub fn part2<I: Iterator<Item = Direction>>(iter: I) -> usize {
    const N: usize = 9;
    let mut head = (0, 0);
    let mut tail = [(0, 0); N];
    let mut tail_positions = HashSet::<(isize, isize)>::new();

    for dir in iter {
        head = step_head(head, dir);

        tail[0] = step_tail(head, tail[0]);

        for i in 1..N {
            tail[i] = step_tail(tail[i - 1], tail[i]);
        }

        //print_rope(head, &tail);
        //println!();

        tail_positions.insert(tail[N - 1]);
    }

    //print_tail(&tail_positions);

    tail_positions.len()
}

fn grid_size<I: Iterator<Item = (isize, isize)>>(iter: I) -> ((isize, isize), (isize, isize)) {
    iter.fold(
        // (Xmin,Ymin),(Xmax,Ymax)
        ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
        |minmax, pos| {
            (
                (minmax.0 .0.min(pos.0), minmax.0 .1.min(pos.1)),
                (minmax.1 .0.max(pos.0), minmax.1 .1.max(pos.1)),
            )
        },
    )
}

fn print_grid<P: Fn(isize, isize) -> char>(size: ((isize, isize), (isize, isize)), f: P) {
    for row in (size.0 .1..(size.1 .1 + 1)).rev() {
        for col in size.0 .0..(size.1 .0 + 1) {
            print!("{}", f(col, row));
        }
        println!();
    }
}

fn print_tail(tail_positions: &HashSet<(isize, isize)>) {
    let size = grid_size(tail_positions.iter().copied());

    println!("{size:?}");

    print_grid(size, |col, row| {
        if col == 0 && row == 0 {
            's'
        } else if tail_positions.contains(&(col, row)) {
            '#'
        } else {
            '.'
        }
    });
}

fn print_rope(head: (isize, isize), tail: &[(isize, isize)]) {
    let size = grid_size(std::iter::once(head).chain(tail.iter().copied()));

    print_grid(size, |col, row| {
        if (col, row) == head {
            'H'
        } else if let Some(tail) = tail
            .iter()
            .enumerate()
            .find(|(_, &t)| t == (col, row))
            .map(|(i, _)| i)
        {
            (tail + 1).to_string().chars().next().unwrap()
        } else {
            '.'
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_example_input1_part1() {
        let iter = parse_input(EXAMPLE_INPUT1);

        assert_eq!(part1(iter), 13);
    }

    #[test]
    fn test_example_input1_part2() {
        let iter = parse_input(EXAMPLE_INPUT1);

        assert_eq!(part2(iter), 1);
    }

    #[test]
    fn test_example_input2_part2() {
        let iter = parse_input(EXAMPLE_INPUT2);

        assert_eq!(part2(iter), 36);
    }
}
