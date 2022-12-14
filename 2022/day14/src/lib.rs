use std::ops::Range;

const SAND_SOURCE: (usize, usize) = (500, 0);
const FLOOR_OFFSET: usize = 2;

type Lines = Vec<Vec<(usize, usize)>>;

#[derive(Debug)]
struct SandSimulator<'a> {
    walls: &'a Lines,
    settled: Vec<(usize, usize)>,
}

impl<'a> SandSimulator<'a> {
    pub fn new(walls: &'a Lines) -> Self {
        Self {
            walls,
            settled: Vec::new(),
        }
    }

    pub fn render<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let bounds = self.settled.iter().fold(self.wall_bounds(), |acc, point| {
            (
                (acc.0 .0.min(point.0), acc.0 .1.max(point.0)),
                (acc.1 .0.min(point.1), acc.1 .1.max(point.1)),
            )
        });

        for y in bounds.1 .0..=(bounds.1 .1 + FLOOR_OFFSET) {
            for x in bounds.0 .0..=bounds.0 .1 {
                if (x, y) == SAND_SOURCE {
                    write!(w, "+")?;
                } else if self.settled.contains(&(x, y)) {
                    write!(w, "o")?;
                } else if self.point_is_wall((x, y)) {
                    write!(w, "#")?;
                } else if y == self.floor_y() {
                    write!(w, "#")?;
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
        let bounds = self.wall_bounds();

        loop {
            let candidates = [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ];

            if stop_out_of_wall_bounds && sand.1 > bounds.1 .1 {
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
                self.settled.push(sand);
                break;
            }
        }

        false
    }

    fn point_is_blocked(&self, point: (usize, usize)) -> bool {
        self.point_is_wall(point) || self.settled.contains(&point) || point.1 == self.floor_y()
    }

    fn point_is_wall(&self, point: (usize, usize)) -> bool {
        self.walls.iter().any(|wall| {
            wall.windows(2)
                .map(|wd| {
                    (
                        (wd[0].0.min(wd[1].0)..=wd[1].0.max(wd[0].0)),
                        (wd[0].1.min(wd[1].1)..=wd[1].1.max(wd[0].1)),
                    )
                })
                .any(|line| line.0.contains(&point.0) && line.1.contains(&point.1))
        })
    }

    fn wall_bounds(&self) -> ((usize, usize), (usize, usize)) {
        self.walls.iter().fold(
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

    fn floor_y(&self) -> usize {
        self.wall_bounds().1 .1 + FLOOR_OFFSET
    }
}

pub fn parse_input(input: &str) -> Lines {
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

pub fn part1(walls: &Lines) -> usize {
    let mut sim = SandSimulator::new(walls);
    let mut counter = 0;

    //sim.render(&mut std::io::stdout()).unwrap();

    while !sim.drop_sand(true) {
        counter += 1;
        //sim.render(&mut std::io::stdout()).unwrap();
    }

    counter
}

pub fn part2(walls: &Lines) -> usize {
    let mut sim = SandSimulator::new(walls);
    let mut counter = 0;

    //sim.render(&mut std::io::stdout()).unwrap();

    while !sim.drop_sand(false) {
        counter += 1;
        //sim.render(&mut std::io::stdout()).unwrap();
    }

    //sim.render(&mut std::io::stdout()).unwrap();

    counter
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
