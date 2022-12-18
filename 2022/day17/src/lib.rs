use std::{collections::HashSet, iter::FromIterator};

const CHAMBER_WIDTH: usize = 7;

mod shapes {
    const MINUS: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
    const PLUS: [(usize, usize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    const L: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    const VERTICAL_LINE: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
    const SQUARE: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

    pub const SHAPES: [&[(usize, usize)]; 5] = [&MINUS, &PLUS, &L, &VERTICAL_LINE, &SQUARE];
}
use shapes::SHAPES;

pub fn part1(input: &str) -> usize {
    let mut occupied = HashSet::<(usize, usize)>::new();

    let mut jets = input.trim_end().as_bytes().iter().cycle();
    let mut counter = 0;

    for shape in SHAPES.iter().cycle().take(2022) {
        let max_y = occupied.iter().fold(0, |acc, p| acc.max(p.1 + 1));
        let mut spawn_pos = (2, max_y + 3);
        let shape_width = shape.iter().fold(0, |acc, p| acc.max(p.0)) + 1;

        if counter < 3 {
            println!("Spawned: {:?}", spawn_pos);
            print_chamber(
                &occupied
                    .union(&HashSet::from_iter(
                        shape.iter().map(|p| (p.0 + spawn_pos.0, p.1 + spawn_pos.1)),
                    ))
                    .cloned()
                    .collect(),
            );
        }

        loop {
            let push_dir = jets.next().unwrap();

            let x_push = match push_dir {
                b'>' => {
                    if spawn_pos.0 + shape_width < CHAMBER_WIDTH {
                        1
                    } else {
                        0
                    }
                }
                b'<' => {
                    if spawn_pos.0 > 0 {
                        -1
                    } else {
                        0
                    }
                }
                c => panic!("invalid push dir: '0x{c:x}'"),
            };

            // check if horizontal movement would cause a collision
            if occupied
                .intersection(&HashSet::from_iter(shape.iter().map(|p| {
                    (
                        ((p.0 + spawn_pos.0) as isize + x_push) as usize,
                        p.1 + spawn_pos.1,
                    )
                })))
                .count()
                == 0
            {
                spawn_pos.0 = (spawn_pos.0 as isize + x_push) as usize;
            }

            if counter < 3 {
                println!("Falling:");
                print_chamber(
                    &occupied
                        .union(&HashSet::from_iter(
                            shape.iter().map(|p| (p.0 + spawn_pos.0, p.1 + spawn_pos.1)),
                        ))
                        .cloned()
                        .collect(),
                );
            }

            // check if downward movement results in collision
            if spawn_pos.1 > 0
                && occupied
                    .intersection(&HashSet::from_iter(
                        shape
                            .iter()
                            .map(|p| (p.0 + spawn_pos.0, p.1 + spawn_pos.1 - 1)),
                    ))
                    .count()
                    == 0
            {
                spawn_pos.1 -= 1;
            } else {
                occupied.extend(shape.iter().map(|p| (p.0 + spawn_pos.0, p.1 + spawn_pos.1)));
                counter += 1;

                if counter < 3 {
                    println!("Settled:");
                    print_chamber(&occupied);
                    println!("=============")
                }

                break;
            }
        }
    }

    occupied.iter().fold(0, |acc, p| acc.max(p.1 + 1))
}

fn print_chamber(occupied: &HashSet<(usize, usize)>) {
    let max_y = occupied.iter().fold(0, |acc, p| acc.max(p.1));

    for y in (0..=max_y).rev() {
        for x in 0..CHAMBER_WIDTH {
            if occupied.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3068);
    }
}
