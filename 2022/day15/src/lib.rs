use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub struct Sensor {
    position: (isize, isize),
    closest_beacon: (isize, isize),
    range: isize,
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

        let position = (sensor_x.parse().unwrap(), sensor_y.parse().unwrap());
        let closest_beacon = (beacon_x.parse().unwrap(), beacon_y.parse().unwrap());
        let range = position.manhattan_distance_to(&closest_beacon);

        Ok(Self {
            position,
            closest_beacon,
            range,
        })
    }
}

impl Sensor {}

pub fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(sensors: &[Sensor], y: isize) -> usize {
    let leftmost = sensors
        .iter()
        .fold(isize::MAX, |acc, s| acc.min(s.position.0 - s.range));
    let rightmost = sensors
        .iter()
        .fold(isize::MIN, |acc, s| acc.max(s.position.0 + s.range));

    let mut count = 0;

    for x in leftmost..=rightmost {
        if sensors.iter().any(|sensor| {
            sensor.position.manhattan_distance_to(&(x, y)) <= sensor.range
                && sensor.closest_beacon != (x, y)
        }) {
            count += 1;
        }
    }

    count
}

pub fn part2(
    sensors: &[Sensor],
    search_space: (RangeInclusive<isize>, RangeInclusive<isize>),
) -> usize {
    let mut beacon = None::<(isize, isize)>;

    'outer: for sensor in sensors {
        // walk around the edge of each sensors max range to reduce the number of
        // possible locations from 16,000,000,000,000 to about 55,746,600
        for dir in [(1, 0), (-1, 0), (0, 1), (1, 0)] {
            let range = sensor.range + 1;

            for i in 0..=range {
                let p = (
                    sensor.position.0 + range * dir.0 - i,
                    sensor.position.1 + range * dir.1 - i,
                );
                if search_space.0.contains(&p.0)
                    && search_space.1.contains(&p.1)
                    && sensors.iter().all(|sensor| {
                        sensor.closest_beacon != p
                            && sensor.position.manhattan_distance_to(&p) > sensor.range
                    })
                {
                    beacon = Some(p);
                    break 'outer;
                }
            }
        }
    }

    if let Some(beacon) = beacon {
        (beacon.0 * 4000000 + beacon.1) as usize
    } else {
        panic!("Couldn't find the distress signal beacon!");
    }
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
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT), 10), 26);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(
            part2(&parse_input(EXAMPLE_INPUT), (0..=20, 0..=20)),
            56000011
        );
    }
}
