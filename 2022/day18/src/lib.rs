use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3D(isize, isize, isize);

impl Point3D {
    pub fn is_adjacent(&self, other: &Point3D) -> bool {
        macro_rules! eq_with_offset {
            ($a:ident, $b:ident, $x:literal, $y:literal, $z:literal) => {
                ($a.0 == $b.0 + $x && $a.1 == $b.1 + $y && $a.2 == $b.2 + $z)
            };
        }

        eq_with_offset!(self, other, 1, 0, 0)
            || eq_with_offset!(other, self, 1, 0, 0)
            || eq_with_offset!(self, other, 0, 1, 0)
            || eq_with_offset!(other, self, 0, 1, 0)
            || eq_with_offset!(self, other, 0, 0, 1)
            || eq_with_offset!(other, self, 0, 0, 1)
    }

    pub fn adjacent(&self) -> [Point3D; 6] {
        let offsets = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        offsets.map(|offset| Point3D(self.0 + offset.0, self.1 + offset.1, self.2 + offset.2))
    }
}

pub fn parse_input(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .map(|line| {
            let (x, rem) = line.split_once(',').unwrap();
            let (y, z) = rem.split_once(',').unwrap();
            Point3D(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect()
}

pub fn part1(cubes: &[Point3D]) -> usize {
    let mut count = 0;
    for cube in cubes {
        let mut free_sides = 6;
        for cube2 in cubes {
            if cube.is_adjacent(cube2) {
                free_sides -= 1;
            }
        }
        count += free_sides;
    }
    count
}

pub fn part2(cubes: &[Point3D]) -> usize {
    let mut grid = HashSet::<Point3D>::new();
    grid.extend(cubes.iter().cloned());

    let lava_bounds = grid.iter().fold(
        (
            (isize::MAX, isize::MIN),
            (isize::MAX, isize::MIN),
            (isize::MAX, isize::MIN),
        ),
        |acc, cube| {
            (
                (acc.0 .0.min(cube.0), acc.0 .1.max(cube.0)),
                (acc.1 .0.min(cube.1), acc.1 .1.max(cube.1)),
                (acc.2 .0.min(cube.2), acc.2 .1.max(cube.2)),
            )
        },
    );

    // encapsulate our lava in water, one block bigger in each direction
    // than the lava bounds
    let start = Point3D(
        lava_bounds.0 .0 - 1,
        lava_bounds.1 .0 - 1,
        lava_bounds.2 .0 - 1,
    );

    // depth first search
    let mut s = Vec::with_capacity(6);
    s.push(start);
    let mut water_blocks = HashSet::new();

    while let Some(v) = s.pop() {
        if !water_blocks.contains(&v) {
            for w in v.adjacent() {
                if w.0 >= lava_bounds.0 .0 - 1
                    && w.0 <= lava_bounds.0 .1 + 1
                    && w.1 >= lava_bounds.1 .0 - 1
                    && w.1 <= lava_bounds.1 .1 + 1
                    && w.2 >= lava_bounds.2 .0 - 1
                    && w.2 <= lava_bounds.2 .1 + 1
                    && !grid.contains(&w)
                {
                    s.push(w);
                }
            }

            water_blocks.insert(v);
        }
    }

    // then see how many water block touch lava
    water_blocks
        .iter()
        .map(|cube| {
            cube.adjacent()
                .iter()
                .filter(|adj| grid.contains(adj))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_example_input_part1() {
        let cubes = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&cubes), 64);
    }

    #[test]
    fn test_example_input_part2() {
        let cubes = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&cubes), 58);
    }
}
