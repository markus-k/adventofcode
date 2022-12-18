#[derive(Debug, PartialEq, Eq)]
pub struct Point3D(usize, usize, usize);

macro_rules! eq_with_offset {
    ($a:ident, $b:ident, $x:literal, $y:literal, $z:literal) => {
        ($a.0 == $b.0 + $x && $a.1 == $b.1 + $y && $a.2 == $b.2 + $z)
    };
}

impl Point3D {
    pub fn is_adjacent(&self, other: &Point3D) -> bool {
        eq_with_offset!(self, other, 1, 0, 0)
            || eq_with_offset!(other, self, 1, 0, 0)
            || eq_with_offset!(self, other, 0, 1, 0)
            || eq_with_offset!(other, self, 0, 1, 0)
            || eq_with_offset!(self, other, 0, 0, 1)
            || eq_with_offset!(other, self, 0, 0, 1)
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
}
