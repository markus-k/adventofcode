use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let field = VentField::from(input);
    let reduced_field = field.with_only_horizontal_or_vertical_vents();
    let map = VentFieldMap::from(&reduced_field);

    let overlaps = map.points_with_overlap(2).len();

    println!("Fields with at least 2 overlaps: {}", overlaps);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct Vent(Point, Point);

impl From<&str> for Vent {
    fn from(input: &str) -> Self {
        lazy_static! {
            // don't compile this regex for every single line
            static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }
        let captures = RE.captures(input).unwrap();

        Self {
            0: Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            1: Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        }
    }
}

impl Vent {
    fn is_horizontal(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_vertical(&self) -> bool {
        self.0.y == self.1.y
    }

    fn points_covered(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let (y0, y1, rev) = if self.0.y < self.1.y {
                (self.0.y, self.1.y, false)
            } else {
                (self.1.y, self.0.y, true)
            };

            let points = (y0..(y1 + 1)).map(|y| Point::new(self.0.x, y));

            if rev {
                points.rev().collect()
            } else {
                points.collect()
            }
        } else if self.is_vertical() {
            let (x0, x1, rev) = if self.0.x < self.1.x {
                (self.0.x, self.1.x, false)
            } else {
                (self.1.x, self.0.x, true)
            };

            let points = (x0..(x1 + 1)).map(|x| Point::new(x, self.0.y));

            if rev {
                points.rev().collect()
            } else {
                points.collect()
            }
        } else {
            panic!("Not implemented");
        }
    }
}

struct VentField {
    vents: Vec<Vent>,
}

impl From<&str> for VentField {
    fn from(input: &str) -> Self {
        let vents = input.lines().map(|line| Vent::from(line)).collect();

        Self { vents }
    }
}

impl VentField {
    fn with_only_horizontal_or_vertical_vents(&self) -> Self {
        let filtered_vents = self
            .vents
            .iter()
            .filter(|vent| vent.is_horizontal() || vent.is_vertical())
            .cloned()
            .collect();

        Self {
            vents: filtered_vents,
        }
    }
}

struct VentFieldMap(HashMap<Point, usize>);

impl From<&VentField> for VentFieldMap {
    fn from(field: &VentField) -> Self {
        let mut map = HashMap::<Point, usize>::new();

        for vent in field.vents.iter() {
            for point in vent.points_covered() {
                map.entry(point)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        Self { 0: map }
    }
}

impl VentFieldMap {
    fn points_with_overlap(&self, overlap: usize) -> Vec<Point> {
        self.0
            .iter()
            .filter_map(|(point, &n)| {
                if n >= overlap {
                    Some(point.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let field = VentField::from(input);
        let reduced_field = field.with_only_horizontal_or_vertical_vents();

        let map = VentFieldMap::from(&reduced_field);
        assert_eq!(map.points_with_overlap(2).len(), 5);
    }

    #[test]
    fn test_parse_vent() {
        let vent_str = "1234,32 -> 1,15";
        let vent = Vent::from(vent_str);

        assert_eq!(vent.0.x, 1234);
        assert_eq!(vent.0.y, 32);
        assert_eq!(vent.1.x, 1);
        assert_eq!(vent.1.y, 15);
    }

    #[test]
    fn test_horizontal_covered_points() {
        let testcases = vec![
            (
                "1,1 -> 1,3",
                vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)],
            ),
            (
                "1,3 -> 1,1",
                vec![Point::new(1, 3), Point::new(1, 2), Point::new(1, 1)],
            ),
        ];
        for (vent, points) in testcases {
            let vent = Vent::from(vent);

            assert!(vent.is_horizontal());
            assert_eq!(vent.points_covered(), points);
        }
    }

    #[test]
    fn test_vertical_covered_points() {
        let testcases = vec![
            (
                "9,7 -> 7,7",
                vec![Point::new(9, 7), Point::new(8, 7), Point::new(7, 7)],
            ),
            (
                "7,7 -> 9,7",
                vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)],
            ),
        ];
        for (vent, points) in testcases {
            let vent = Vent::from(vent);

            assert!(vent.is_vertical());
            assert_eq!(vent.points_covered(), points);
        }
    }
}
