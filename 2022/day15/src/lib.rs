use std::str::FromStr;

#[derive(Debug)]
pub struct Sensor {
    position: (isize, isize),
    closest_beacon: (isize, isize),
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_x, rem) = s
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();
        let (sensor_y, rem) = rem.split_once(": closest beacon is at x=").unwrap();
        let (beacon_x, beacon_y) = rem.split_once(", y=").unwrap();

        Ok(Self {
            position: (sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
            closest_beacon: (beacon_x.parse().unwrap(), beacon_y.parse().unwrap()),
        })
    }
}

impl Sensor {
    fn range(&self) -> isize {
        self.position.manhattan_distance_to(&self.closest_beacon)
    }
}

pub fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(sensors: &[Sensor], y: isize) -> usize {
    let leftmost = sensors
        .iter()
        .fold(isize::MAX, |acc, s| acc.min(s.position.0 - s.range()));
    let rightmost = sensors
        .iter()
        .fold(isize::MIN, |acc, s| acc.max(s.position.0 + s.range()));

    let mut count = 0;

    for x in leftmost..=rightmost {
        if sensors.iter().any(|sensor| {
            sensor.position.manhattan_distance_to(&(x, y)) <= sensor.range()
                && sensor.closest_beacon != (x, y)
        }) {
            count += 1;
        }
    }

    count
}

trait ManhattanDistance {
    type Output;

    fn manhattan_distance_to(&self, to: &Self) -> Self::Output;
}

impl ManhattanDistance for (isize, isize) {
    type Output = isize;

    fn manhattan_distance_to(&self, to: &Self) -> Self::Output {
        (self.0 - to.0).abs() + (self.1 - to.1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_example_input_part1() {
        assert_eq!(part1(&dbg!(parse_input(EXAMPLE_INPUT)), 10), 26);
    }
}
