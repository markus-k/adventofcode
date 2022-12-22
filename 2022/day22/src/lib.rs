use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Tile {
    Void,
    Open,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    tiles: HashMap<(isize, isize), Tile>,
    path: String,
}

impl Map {
    fn start(&self) -> (isize, isize) {
        *self
            .tiles
            .iter()
            .filter(|&(p, t)| p.1 == 0 && *t == Tile::Open)
            .min_by_key(|(p, _t)| p.0)
            .map(|(p, _t)| p)
            .unwrap()
    }
}

pub fn parse_input(input: &str) -> Map {
    let (map, path) = input.split_once("\n\n").unwrap();

    let tiles = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes().iter().enumerate().map(move |(x, c)| {
                (
                    (x as isize, y as isize),
                    match c {
                        b' ' => Tile::Void,
                        b'.' => Tile::Open,
                        b'#' => Tile::Wall,
                        _ => panic!("Invalid tile '{c}'"),
                    },
                )
            })
        })
        .collect::<HashMap<_, _>>();

    Map {
        tiles,
        path: path.to_owned(),
    }
}

#[derive(Debug)]
enum PathSegment {
    TurnLeft,
    TurnRight,
    Move(isize),
}

fn parse_path(path: &str) -> (&str, Option<PathSegment>) {
    match path.as_bytes().first() {
        Some(b'L') => (&path[1..], Some(PathSegment::TurnLeft)),
        Some(b'R') => (&path[1..], Some(PathSegment::TurnRight)),
        Some(n) if n.is_ascii_digit() => {
            let (num, rem) = path.split_at(
                path.chars()
                    .position(|c: char| !c.is_ascii_digit())
                    .unwrap_or(path.len()),
            );
            (rem, Some(PathSegment::Move(num.parse().unwrap())))
        }
        _ => (path, None),
    }
}

pub fn part1(map: &Map) -> isize {
    let mut dir = 0;
    let mut pos = map.start();

    let (mut path_rem, mut path_seg) = parse_path(&map.path);

    while let Some(seg) = path_seg {
        match seg {
            PathSegment::TurnLeft => {
                if dir == 0 {
                    dir = 3
                } else {
                    dir -= 1
                }
            }
            PathSegment::TurnRight => {
                if dir == 3 {
                    dir = 0
                } else {
                    dir += 1
                }
            }
            PathSegment::Move(steps) => {
                let delta = match dir {
                    0 => (1, 0),
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    _ => unreachable!("invalid direction '{dir}'"),
                };

                for _ in 0..steps {
                    let newpos = (pos.0 + delta.0, pos.1 + delta.1);

                    match map.tiles.get(&newpos) {
                        Some(Tile::Void) | None => {
                            println!("almost fell into the void");

                            let mut wrappos = pos;
                            loop {
                                wrappos = (wrappos.0 - delta.0, wrappos.1 - delta.1);
                                match map.tiles.get(&wrappos) {
                                    Some(Tile::Void) | None => {
                                        println!("found the other side");
                                        //pos = (wrappos.0 + delta.0, wrappos.1 + delta.1);
                                        break;
                                    }
                                    Some(Tile::Open) | Some(Tile::Wall) => {
                                        println!("wrapping around... {wrappos:?}");
                                    }
                                }
                            }

                            let newpos = (wrappos.0 + delta.0, wrappos.1 + delta.1);
                            if map.tiles.get(&newpos) != Some(&Tile::Wall) {
                                pos = newpos;
                            } else {
                                println!("There's a wall on the other side.. staying at {pos:?} instead of going to {newpos:?}");
                            }
                        }
                        Some(Tile::Wall) => {
                            println!("ran into a wall at {newpos:?}");
                        }
                        Some(Tile::Open) => {
                            pos = newpos;
                        }
                    };
                    dbg!(&pos, &dir);
                }
            }
        }

        (path_rem, path_seg) = parse_path(path_rem);
    }

    (pos.0 + 1) * 4 + (pos.1 + 1) * 1000 + dir
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_example_input_part1() {
        let map = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&map), 6032);
    }
}
