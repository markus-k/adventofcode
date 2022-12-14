use std::{collections::HashSet, ops::RangeInclusive};

const SAND_SOURCE: (usize, usize) = (500, 0);
const FLOOR_OFFSET: usize = 2;

#[derive(Debug)]
struct SandSimulator {
    settled: HashSet<(usize, usize)>,

    wall_bounds: (RangeInclusive<usize>, RangeInclusive<usize>),
    obstacle_map: Vec<bool>,
    obstacle_map_size: (usize, usize),
}

impl SandSimulator {
    pub fn new(walls: &[Vec<(usize, usize)>]) -> Self {
        let wall_bounds = wall_bounds(walls);
        let wall_bounds = (
            wall_bounds.0 .0..=wall_bounds.0 .1,
            wall_bounds.1 .0..=wall_bounds.1 .1,
        );
        let obstacle_map_size = (
            wall_bounds.0.end() - wall_bounds.0.start() + 1,
            wall_bounds.1.end() - wall_bounds.1.start() + 1,
        );

        let mut obstacle_map = vec![false; obstacle_map_size.0 * obstacle_map_size.1];
        for wall in walls {
            for wd in wall.windows(2) {
                if wd[0].0 == wd[1].0 {
                    // vertical
                    for y in wd[0].1.min(wd[1].1)..=(wd[1].1.max(wd[0].1) + 0) {
                        obstacle_map[calc_obstacle_map_index(
                            obstacle_map_size,
                            &wall_bounds,
                            (wd[0].0, y),
                        )] = true;
                    }
                } else if wd[0].1 == wd[1].1 {
                    // horizontal
                    for x in wd[0].0.min(wd[1].0)..=(wd[1].0.max(wd[0].0) + 0) {
                        obstacle_map[calc_obstacle_map_index(
                            obstacle_map_size,
                            &wall_bounds,
                            (x, wd[0].1),
                        )] = true;
                    }
                } else {
                    panic!("Only vertical and horizontal walls are supported");
                }
            }
        }

        Self {
            settled: HashSet::new(),
            wall_bounds,
            obstacle_map,
            obstacle_map_size,
        }
    }

    pub fn render<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let bounds = self
            .settled
            .iter()
            .fold(self.wall_bounds.clone(), |acc, point| {
                (
                    (*(acc.0.start()).min(&point.0)..=*(acc.0.end()).max(&point.0)),
                    (*(acc.1.start()).min(&point.1)..=*(acc.1.end()).max(&point.1)),
                )
            });

        for y in 0..=(bounds.1.end() + FLOOR_OFFSET) {
            for x in bounds.0.clone() {
                if (x, y) == SAND_SOURCE {
                    write!(w, "+")?;
                } else if self.settled.contains(&(x, y)) {
                    write!(w, "o")?;
                } else if self.point_is_wall((x, y)) {
                    write!(w, "#")?;
                } else if y == self.floor_y() {
                    write!(w, "=")?;
                } else {
                    write!(w, ".")?;
                }
            }
            writeln!(w)?;
        }

        Ok(())
    }

    pub fn drop_sand(&mut self, stop_out_of_wall_bounds: bool) -> bool {
        let mut sand = SAND_SOURCE;

        loop {
            let candidates = [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ];

            if stop_out_of_wall_bounds && sand.1 > *self.wall_bounds.1.end() {
                // fell out of the world
                return true;
            }

            if self.settled.contains(&SAND_SOURCE) {
                // source is clogged
                return true;
            }

            if let Some(next) = candidates
                .iter()
                .find(|&next_point| !self.point_is_blocked(*next_point))
            {
                sand = *next;
            } else {
                self.settled.insert(sand);
                break;
            }
        }

        false
    }

    fn point_is_blocked(&self, point: (usize, usize)) -> bool {
        self.point_is_wall(point) || point.1 == self.floor_y() || self.settled.contains(&point)
    }

    fn point_is_wall(&self, point: (usize, usize)) -> bool {
        self.wall_bounds.0.contains(&point.0)
            && self.wall_bounds.1.contains(&point.1)
            && self.obstacle_map[self.obstacle_map_index(point)]
    }

    fn obstacle_map_index(&self, point: (usize, usize)) -> usize {
        calc_obstacle_map_index(self.obstacle_map_size, &self.wall_bounds, point)
    }

    fn floor_y(&self) -> usize {
        self.wall_bounds.1.end() + FLOOR_OFFSET
    }
}

fn calc_obstacle_map_index(
    size: (usize, usize),
    wall_bounds: &(RangeInclusive<usize>, RangeInclusive<usize>),
    point: (usize, usize),
) -> usize {
    size.0 * (point.1 - wall_bounds.1.start()) + point.0 - wall_bounds.0.start()
}

fn wall_bounds(walls: &[Vec<(usize, usize)>]) -> ((usize, usize), (usize, usize)) {
    walls.iter().fold(
        ((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)),
        |acc, line| {
            line.iter().fold(acc, |acc, point| {
                (
                    (acc.0 .0.min(point.0), acc.0 .1.max(point.0)),
                    (acc.1 .0.min(point.1), acc.1 .1.max(point.1)),
                )
            })
        },
    )
}

pub fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split_once(',')
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .collect()
        })
        .collect::<Vec<_>>()
}

pub fn part1(walls: &[Vec<(usize, usize)>]) -> usize {
    simulate(walls, true, SimOptions::new())
}

pub fn part2(walls: &[Vec<(usize, usize)>]) -> usize {
    simulate(walls, false, SimOptions::new())
}

fn simulate(walls: &[Vec<(usize, usize)>], overflow: bool, options: SimOptions) -> usize {
    let mut sim = SandSimulator::new(walls);
    let mut counter = 0;

    if options.print_start {
        sim.render(&mut std::io::stdout()).unwrap();
    }

    while !sim.drop_sand(overflow) {
        counter += 1;
        if options.print_steps {
            sim.render(&mut std::io::stdout()).unwrap();
        }
    }

    if options.print_result {
        sim.render(&mut std::io::stdout()).unwrap();
    }

    counter
}

struct SimOptions {
    print_start: bool,
    print_steps: bool,
    print_result: bool,
}

// a builder pattern.. just for fun

macro_rules! builder_option {
    ($field:ident, $type:ident) => {
        pub fn $field(self, $field: $type) -> Self {
            Self { $field, ..self }
        }
    };
}

impl SimOptions {
    pub fn new() -> Self {
        Self {
            print_start: false,
            print_steps: false,
            print_result: false,
        }
    }

    builder_option!(print_start, bool);
    builder_option!(print_steps, bool);
    builder_option!(print_result, bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_example_input_part1() {
        let walls = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&walls), 24);
    }

    #[test]
    fn test_example_input_part2() {
        let walls = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&walls), 93);
    }
}
